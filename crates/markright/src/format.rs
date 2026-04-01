use std::fmt::{self, Write};

use crate::ast::block::{Block, Document};
use crate::ast::common::Alignment;
use crate::ast::inline::Inline;

pub fn to_string(doc: &Document) -> String {
    let mut buf = String::new();
    write_doc(doc, &mut buf).unwrap();
    buf
}

pub fn write_doc(doc: &Document, out: &mut dyn Write) -> fmt::Result {
    if let Some(fm) = &doc.front_matter {
        out.write_str("---\n")?;
        out.write_str(fm.raw)?;
        out.write_str("\n---\n")?;
    }
    // If no front matter and first block is ThematicBreak, prepend a blank
    // line so "---" at position 0 isn't misinterpreted as front matter.
    if doc.front_matter.is_none()
        && doc
            .children
            .first()
            .is_some_and(|b| matches!(b, Block::ThematicBreak {}))
    {
        out.write_char('\n')?;
    }
    for (i, block) in doc.children.iter().enumerate() {
        if i > 0 || doc.front_matter.is_some() {
            out.write_char('\n')?;
        }
        write_block(block, out)?;
    }
    Ok(())
}

fn write_block(block: &Block, out: &mut dyn Write) -> fmt::Result {
    match block {
        Block::Heading { level, id, content } => {
            for _ in 0..*level {
                out.write_char('#')?;
            }
            out.write_char(' ')?;
            write_inlines(content, out)?;
            write_opt_id(id, out)?;
            out.write_char('\n')
        }
        Block::Paragraph { id, content } => {
            let mut buf = String::new();
            write_inlines(content, &mut buf)?;
            let trimmed = buf.trim();
            if needs_block_escape(trimmed) {
                out.write_char('\\')?;
            }
            out.write_str(trimmed)?;
            write_opt_id(id, out)?;
            out.write_char('\n')
        }
        Block::CodeBlock { lang, body, id: _ } => {
            let fence = backtick_fence(body, 3);
            out.write_str(&fence)?;
            if let Some(lang) = lang {
                out.write_str(lang)?;
            }
            out.write_char('\n')?;
            out.write_str(body)?;
            out.write_char('\n')?;
            out.write_str(&fence)?;
            out.write_char('\n')
        }
        Block::DiagramBlock { lang, body, id: _ } => {
            let fence = backtick_fence(body, 3);
            out.write_str(&fence)?;
            out.write_str(lang)?;
            out.write_char('\n')?;
            out.write_str(body)?;
            out.write_char('\n')?;
            out.write_str(&fence)?;
            out.write_char('\n')
        }
        Block::MathBlock { body, id: _ } => {
            out.write_str("$$\n")?;
            out.write_str(body)?;
            out.write_str("\n$$\n")
        }
        Block::ThematicBreak {} => out.write_str("---\n"),
        Block::Blockquote { children } => {
            if children.is_empty() {
                return out.write_str(">\n");
            }
            let inner = format_children(children);
            for line in inner.lines() {
                if line.is_empty() {
                    out.write_str(">\n")?;
                } else {
                    writeln!(out, "> {line}")?;
                }
            }
            Ok(())
        }
        Block::UnorderedList { items } => {
            for item in items {
                out.write_str("- ")?;
                write_list_body(&item.children, "  ", out)?;
            }
            Ok(())
        }
        Block::OrderedList { start, items } => {
            for (i, item) in items.iter().enumerate() {
                let num = *start + i as u32;
                let prefix = format!("{num}. ");
                out.write_str(&prefix)?;
                let indent = " ".repeat(prefix.len());
                write_list_body(&item.children, &indent, out)?;
            }
            Ok(())
        }
        Block::TaskList { items } => {
            for item in items {
                let marker = item.state.to_char();
                write!(out, "- [{marker}] ")?;
                write_list_body(&item.children, "  ", out)?;
            }
            Ok(())
        }
        Block::Table {
            headers,
            alignments,
            rows,
            caption,
            id,
        } => {
            // Header row
            out.write_char('|')?;
            for header in headers {
                out.write_char(' ')?;
                write_inlines(header, out)?;
                out.write_str(" |")?;
            }
            out.write_char('\n')?;
            // Separator row
            out.write_char('|')?;
            for (i, _) in headers.iter().enumerate() {
                let align = alignments.get(i).copied().unwrap_or(Alignment::None);
                match align {
                    Alignment::None => out.write_str(" --- ")?,
                    Alignment::Left => out.write_str(" :--- ")?,
                    Alignment::Center => out.write_str(" :---: ")?,
                    Alignment::Right => out.write_str(" ---: ")?,
                };
                out.write_char('|')?;
            }
            out.write_char('\n')?;
            // Data rows
            for row in rows {
                out.write_char('|')?;
                for cell in row {
                    out.write_char(' ')?;
                    write_inlines(cell, out)?;
                    out.write_str(" |")?;
                }
                out.write_char('\n')?;
            }
            // Caption
            if let Some(cap) = caption {
                out.write_char('\n')?;
                out.write_str("Table: ")?;
                write_inlines(cap, out)?;
                write_opt_id(id, out)?;
                out.write_char('\n')?;
            }
            Ok(())
        }
        Block::Admonition {
            kind,
            foldable,
            children,
        } => {
            let prefix = kind.prefix();
            let inner = format_children(children);
            let lines: Vec<&str> = inner.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                write!(out, "{prefix}>")?;
                if i == 0 && *foldable {
                    out.write_char('-')?;
                }
                if !line.is_empty() {
                    out.write_char(' ')?;
                    out.write_str(line)?;
                }
                out.write_char('\n')?;
            }
            if lines.is_empty() {
                write!(out, "{prefix}>")?;
                if *foldable {
                    out.write_char('-')?;
                }
                out.write_char('\n')?;
            }
            Ok(())
        }
        Block::DefinitionList { items } => {
            for item in items {
                write_inlines(&item.term, out)?;
                out.write_char('\n')?;
                for def in &item.definitions {
                    out.write_str(": ")?;
                    write_inlines(def, out)?;
                    out.write_char('\n')?;
                }
            }
            Ok(())
        }
        Block::FencedDiv { name, children } => {
            let fence = div_fence_for(children);
            out.write_str(&fence)?;
            if let Some(name) = name {
                out.write_str(name)?;
            }
            out.write_char('\n')?;
            for (i, child) in children.iter().enumerate() {
                if i > 0 {
                    out.write_char('\n')?;
                }
                write_block(child, out)?;
            }
            out.write_str(&fence)?;
            out.write_char('\n')
        }
        Block::FootnoteDef { label, children } => {
            write!(out, "[^{label}]: ")?;
            let inner = format_children(children);
            let mut lines = inner.lines();
            if let Some(first) = lines.next() {
                out.write_str(first)?;
                out.write_char('\n')?;
            }
            for line in lines {
                if line.is_empty() {
                    out.write_char('\n')?;
                } else {
                    writeln!(out, "    {line}")?;
                }
            }
            Ok(())
        }
        Block::Comment { body } => {
            if body.contains('\n') {
                out.write_str("%%\n")?;
                out.write_str(body)?;
                out.write_str("\n%%\n")
            } else {
                writeln!(out, "%% {body} %%")
            }
        }
        Block::Toc {
            min_level,
            max_level,
        } => {
            if *min_level == 1 && *max_level == 6 {
                out.write_str("[TOC]\n")
            } else {
                writeln!(out, "[TOC:{min_level}-{max_level}]")
            }
        }
        Block::Include { path, fragment } => {
            out.write_str("{{")?;
            out.write_str(path)?;
            if let Some(frag) = fragment {
                out.write_char('#')?;
                out.write_str(frag)?;
            }
            out.write_str("}}\n")
        }
    }
}

fn write_inlines(inlines: &[Inline], out: &mut dyn Write) -> fmt::Result {
    for node in inlines {
        write_inline(node, out)?;
    }
    Ok(())
}

fn write_inline(node: &Inline, out: &mut dyn Write) -> fmt::Result {
    match node {
        Inline::Text { value } => escape_source(out, value),
        Inline::Bold { children } => {
            out.write_str("**")?;
            write_inlines(children, out)?;
            out.write_str("**")
        }
        Inline::Italic { children } => {
            out.write_char('*')?;
            write_inlines(children, out)?;
            out.write_char('*')
        }
        Inline::BoldItalic { children } => {
            out.write_str("***")?;
            write_inlines(children, out)?;
            out.write_str("***")
        }
        Inline::Strikethrough { children } => {
            out.write_str("~~")?;
            write_inlines(children, out)?;
            out.write_str("~~")
        }
        Inline::Highlight { children } => {
            out.write_str("==")?;
            write_inlines(children, out)?;
            out.write_str("==")
        }
        Inline::Superscript { children } => {
            out.write_char('^')?;
            write_inlines(children, out)?;
            out.write_char('^')
        }
        Inline::Subscript { children } => {
            out.write_char('~')?;
            write_inlines(children, out)?;
            out.write_char('~')
        }
        Inline::InlineCode { value } => {
            let ticks = backtick_fence(value, 1);
            out.write_str(&ticks)?;
            out.write_str(value)?;
            out.write_str(&ticks)
        }
        Inline::InlineMath { value } => {
            out.write_char('$')?;
            out.write_str(value)?;
            out.write_char('$')
        }
        Inline::Link {
            url,
            title,
            children,
        } => {
            // Check if this is an autolink (link text == url, http(s) only)
            if title.is_none()
                && (url.starts_with("https://") || url.starts_with("http://"))
                && children.len() == 1
                && let Inline::Text { value } = &children[0]
                && *value == *url
            {
                return out.write_str(url);
            }
            out.write_char('[')?;
            write_inlines(children, out)?;
            out.write_str("](")?;
            out.write_str(url)?;
            if let Some(title) = title {
                write!(out, " \"{title}\"")?;
            }
            out.write_char(')')
        }
        Inline::Image {
            url,
            alt,
            title,
            attrs,
        } => {
            write!(out, "![{alt}]({url}")?;
            if let Some(title) = title {
                write!(out, " \"{title}\"")?;
            }
            out.write_char(')')?;
            if !attrs.is_empty() {
                out.write_char('{')?;
                for (i, (k, v)) in attrs.iter().enumerate() {
                    if i > 0 {
                        out.write_char(' ')?;
                    }
                    write!(out, "{k}={v}")?;
                }
                out.write_char('}')?;
            }
            Ok(())
        }
        Inline::WikiLink {
            target,
            fragment,
            alias,
        } => {
            out.write_str("[[")?;
            out.write_str(target)?;
            if let Some(frag) = fragment {
                out.write_char('#')?;
                out.write_str(frag)?;
            }
            if let Some(alias) = alias {
                out.write_char('|')?;
                out.write_str(alias)?;
            }
            out.write_str("]]")
        }
        Inline::WikiEmbed {
            target,
            fragment,
            width,
            height,
        } => {
            out.write_str("![[")?;
            out.write_str(target)?;
            if let Some(frag) = fragment {
                out.write_char('#')?;
                out.write_str(frag)?;
            }
            if let Some(w) = width {
                out.write_char('|')?;
                write!(out, "{w}")?;
                if let Some(h) = height {
                    write!(out, "x{h}")?;
                }
            }
            out.write_str("]]")
        }
        Inline::BracketedSpan { children, attrs } => {
            out.write_char('[')?;
            write_inlines(children, out)?;
            out.write_str("]{")?;
            out.write_str(attrs)?;
            out.write_char('}')
        }
        Inline::FootnoteRef { label } => write!(out, "[^{label}]"),
        Inline::RangeFootnote { label, children } => {
            write!(out, "[^{label}>")?;
            write_inlines(children, out)?;
            out.write_char(']')
        }
        Inline::InlineFootnote { children } => {
            out.write_str("^[")?;
            write_inlines(children, out)?;
            out.write_char(']')
        }
        Inline::Citation { keys } => {
            out.write_char('[')?;
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    out.write_str("; ")?;
                }
                if key.suppress_author {
                    out.write_str("-@")?;
                } else {
                    out.write_char('@')?;
                }
                out.write_str(key.key)?;
                if let Some(loc) = key.locator {
                    write!(out, ", {loc}")?;
                }
            }
            out.write_char(']')
        }
        Inline::HardBreak {} => out.write_str("\\\n"),
    }
}

/// Check if paragraph text starts with a character that would trigger block parsing.
fn needs_block_escape(text: &str) -> bool {
    let bytes = text.as_bytes();
    let Some(&first) = bytes.first() else {
        return false;
    };
    match first {
        b'#' | b'>' | b':' | b'%' | b'{' => true,
        b'-' => {
            bytes.get(1) == Some(&b' ')
                || bytes.get(1) == Some(&b'-')
                || bytes.get(1) == Some(&b'[')
        }
        b'N' | b'T' | b'W' | b'X' => bytes.get(1) == Some(&b'>'),
        b'0'..=b'9' => text.contains(". "),
        _ => false,
    }
}

/// Escape inline marker characters so text round-trips through parse -> format.
fn escape_source(out: &mut dyn Write, s: &str) -> fmt::Result {
    let bytes = s.as_bytes();
    let mut last = 0;
    for (i, &b) in bytes.iter().enumerate() {
        let needs_escape = match b {
            b'\\' | b'`' | b'$' | b'*' | b'[' | b']' | b'^' | b'~' | b'=' | b'!' => true,
            b'h' => s[i..].starts_with("https://") || s[i..].starts_with("http://"),
            _ => false,
        };
        if needs_escape {
            out.write_str(&s[last..i])?;
            out.write_char('\\')?;
            last = i;
        }
    }
    out.write_str(&s[last..])
}

fn write_opt_id(id: &Option<&str>, out: &mut dyn Write) -> fmt::Result {
    if let Some(id) = id {
        write!(out, " {{#{id}}}")?;
    }
    Ok(())
}

/// Compute the minimum colon fence length that doesn't conflict with nested divs.
fn div_fence_for(children: &[Block]) -> String {
    let depth = max_div_depth(children);
    ":".repeat(3 + depth)
}

fn max_div_depth(blocks: &[Block]) -> usize {
    let mut max = 0;
    for block in blocks {
        if let Block::FencedDiv { children, .. } = block {
            let d = 1 + max_div_depth(children);
            if d > max {
                max = d;
            }
        }
    }
    max
}

/// Compute a backtick string that doesn't conflict with content.
/// `min` is the minimum number of backticks (3 for code blocks, 1 for inline code).
fn backtick_fence(body: &str, min: usize) -> String {
    let mut max_run: usize = 0;
    let mut run: usize = 0;
    for b in body.bytes() {
        if b == b'`' {
            run += 1;
            max_run = max_run.max(run);
        } else {
            run = 0;
        }
    }
    "`".repeat((max_run + 1).max(min))
}

/// Format children blocks to a string for re-indenting (blockquotes, admonitions).
fn format_children(children: &[Block]) -> String {
    let mut buf = String::new();
    for (i, child) in children.iter().enumerate() {
        if i > 0 {
            buf.push('\n');
        }
        write_block(child, &mut buf).unwrap();
    }
    buf
}

/// Write list item body, indenting continuation lines.
fn write_list_body(children: &[Block], indent: &str, out: &mut dyn Write) -> fmt::Result {
    let inner = format_children(children);
    let mut lines = inner.lines();
    if let Some(first) = lines.next() {
        out.write_str(first)?;
        out.write_char('\n')?;
    }
    for line in lines {
        if line.is_empty() {
            out.write_char('\n')?;
        } else {
            out.write_str(indent)?;
            out.write_str(line)?;
            out.write_char('\n')?;
        }
    }
    Ok(())
}

use std::collections::HashMap;
use std::fmt::{self, Write};

use crate::ast::block::{Block, Document};
use crate::ast::common::{AdmonitionKind, Alignment, TaskState};
use crate::ast::inline::Inline;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct HtmlOptions {
    pub wikilinks: HashMap<String, ResolvedWikiLink>,
    pub embeds: HashMap<String, ResolvedEmbed>,
    pub classes: ClassMap,
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ResolvedWikiLink {
    pub href: Option<String>,
    pub display: Option<String>,
    pub class: Option<String>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ResolvedEmbed {
    pub html: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ClassMap {
    pub wikilink: String,
    pub wikilink_broken: String,
    pub wikiembed: String,
    pub task_list: String,
    pub task_icon: String,
    pub task_content: String,
    pub math_display: String,
    pub math_inline: String,
    pub admonition: String,
    pub footnote: String,
    pub footnote_ref: String,
    pub footnote_range: String,
    pub footnote_inline: String,
    pub highlight: String,
}

impl Default for ClassMap {
    fn default() -> Self {
        Self {
            wikilink: "wikilink".into(),
            wikilink_broken: "wikilink-broken".into(),
            wikiembed: "wikiembed".into(),
            task_list: "task-list".into(),
            task_icon: "task-icon".into(),
            task_content: "task-content".into(),
            math_display: "math-display".into(),
            math_inline: "math-inline".into(),
            admonition: "admonition".into(),
            footnote: "footnote".into(),
            footnote_ref: "footnote-ref".into(),
            footnote_range: "footnote-range".into(),
            footnote_inline: "footnote-inline".into(),
            highlight: "highlight".into(),
        }
    }
}

pub fn to_html(doc: &Document) -> String {
    let mut buf = String::new();
    write_html(doc, &mut buf).unwrap();
    buf
}

pub fn to_html_with_options(doc: &Document, opts: &HtmlOptions) -> String {
    let mut buf = String::new();
    write_html_with_options(doc, opts, &mut buf).unwrap();
    buf
}

pub fn write_html(doc: &Document, out: &mut dyn Write) -> fmt::Result {
    write_html_with_options(doc, &HtmlOptions::default(), out)
}

pub fn write_html_with_options(
    doc: &Document,
    opts: &HtmlOptions,
    out: &mut dyn Write,
) -> fmt::Result {
    for block in &doc.children {
        write_block(block, opts, out)?;
    }
    Ok(())
}

fn escape(out: &mut dyn Write, s: &str) -> fmt::Result {
    for c in s.chars() {
        match c {
            '&' => out.write_str("&amp;")?,
            '<' => out.write_str("&lt;")?,
            '>' => out.write_str("&gt;")?,
            '"' => out.write_str("&quot;")?,
            _ => out.write_char(c)?,
        }
    }
    Ok(())
}

fn escape_href(out: &mut dyn Write, s: &str) -> fmt::Result {
    for c in s.chars() {
        match c {
            '&' => out.write_str("&amp;")?,
            '"' => out.write_str("%22")?,
            _ => out.write_char(c)?,
        }
    }
    Ok(())
}

fn write_block(block: &Block, opts: &HtmlOptions, out: &mut dyn Write) -> fmt::Result {
    match block {
        Block::Heading { level, id, content } => {
            write!(out, "<h{level}")?;
            write_opt_id(id, out)?;
            out.write_char('>')?;
            write_inlines(content, opts, out)?;
            writeln!(out, "</h{level}>")
        }
        Block::Paragraph { id, content } => {
            out.write_str("<p")?;
            write_opt_id(id, out)?;
            out.write_char('>')?;
            write_inlines(content, opts, out)?;
            out.write_str("</p>\n")
        }
        Block::CodeBlock { lang, body, id } => {
            out.write_str("<pre")?;
            write_opt_id(id, out)?;
            out.write_str("><code")?;
            if let Some(lang) = lang {
                out.write_str(" class=\"language-")?;
                escape(out, lang)?;
                out.write_char('"')?;
            }
            out.write_char('>')?;
            escape(out, body)?;
            out.write_str("</code></pre>\n")
        }
        Block::DiagramBlock { lang, body, id } => {
            out.write_str("<pre")?;
            write_opt_id(id, out)?;
            out.write_str("><code class=\"language-")?;
            escape(out, lang)?;
            out.write_str("\">")?;
            escape(out, body)?;
            out.write_str("</code></pre>\n")
        }
        Block::MathBlock { body, id } => {
            write!(out, "<div class=\"{}\"", opts.classes.math_display)?;
            write_opt_id(id, out)?;
            out.write_str(">\\[")?;
            escape(out, body)?;
            out.write_str("\\]</div>\n")
        }
        Block::ThematicBreak {} => out.write_str("<hr>\n"),
        Block::Blockquote { children } => {
            out.write_str("<blockquote>\n")?;
            for child in children {
                write_block(child, opts, out)?;
            }
            out.write_str("</blockquote>\n")
        }
        Block::UnorderedList { items } => {
            out.write_str("<ul>\n")?;
            for item in items {
                out.write_str("<li>")?;
                write_block_children(&item.children, opts, out)?;
                out.write_str("</li>\n")?;
            }
            out.write_str("</ul>\n")
        }
        Block::OrderedList { start, items } => {
            if *start == 1 {
                out.write_str("<ol>\n")?;
            } else {
                writeln!(out, "<ol start=\"{start}\">")?;
            }
            for item in items {
                out.write_str("<li>")?;
                write_block_children(&item.children, opts, out)?;
                out.write_str("</li>\n")?;
            }
            out.write_str("</ol>\n")
        }
        Block::TaskList { items } => {
            writeln!(out, "<ul class=\"{}\">", opts.classes.task_list)?;
            for item in items {
                let (label, icon) = match item.state {
                    TaskState::Open => ("open", "\u{25cb}"),
                    TaskState::Done => ("done", "\u{2713}"),
                    TaskState::Active => ("active", "\u{25cf}"),
                    TaskState::Review => ("review", "\u{25c9}"),
                    TaskState::Cancelled => ("cancelled", "\u{2715}"),
                    TaskState::Blocked => ("blocked", "\u{26a0}"),
                    TaskState::Deferred => ("deferred", "\u{25b7}"),
                    TaskState::Question => ("question", "?"),
                };
                write!(out, "<li data-state=\"{label}\">")?;
                write!(
                    out,
                    "<span class=\"{}\">{icon}</span>",
                    opts.classes.task_icon
                )?;
                write!(out, "<span class=\"{}\">", opts.classes.task_content)?;
                write_block_children(&item.children, opts, out)?;
                out.write_str("</span></li>\n")?;
            }
            out.write_str("</ul>\n")
        }
        Block::Table {
            headers,
            alignments,
            rows,
            caption,
            id,
        } => {
            out.write_str("<table")?;
            write_opt_id(id, out)?;
            out.write_str(">\n")?;
            if let Some(cap) = caption {
                out.write_str("<caption>")?;
                write_inlines(cap, opts, out)?;
                out.write_str("</caption>\n")?;
            }
            out.write_str("<thead>\n<tr>\n")?;
            for (i, header) in headers.iter().enumerate() {
                write_table_cell("th", alignments.get(i).copied(), header, opts, out)?;
            }
            out.write_str("</tr>\n</thead>\n")?;
            if !rows.is_empty() {
                out.write_str("<tbody>\n")?;
                for row in rows {
                    out.write_str("<tr>\n")?;
                    for (i, cell) in row.iter().enumerate() {
                        write_table_cell("td", alignments.get(i).copied(), cell, opts, out)?;
                    }
                    out.write_str("</tr>\n")?;
                }
                out.write_str("</tbody>\n")?;
            }
            out.write_str("</table>\n")
        }
        Block::Admonition {
            kind,
            foldable,
            children,
        } => {
            let kind_class = match kind {
                AdmonitionKind::Note => "note",
                AdmonitionKind::Tip => "tip",
                AdmonitionKind::Warning => "warning",
                AdmonitionKind::Important => "important",
                AdmonitionKind::Caution => "caution",
            };
            if *foldable {
                writeln!(
                    out,
                    "<details class=\"{} {kind_class}\">",
                    opts.classes.admonition
                )?;
                writeln!(out, "<summary>{kind_class}</summary>")?;
                for child in children {
                    write_block(child, opts, out)?;
                }
                out.write_str("</details>\n")
            } else {
                writeln!(
                    out,
                    "<div class=\"{} {kind_class}\">",
                    opts.classes.admonition
                )?;
                for child in children {
                    write_block(child, opts, out)?;
                }
                out.write_str("</div>\n")
            }
        }
        Block::DefinitionList { items } => {
            out.write_str("<dl>\n")?;
            for item in items {
                out.write_str("<dt>")?;
                write_inlines(&item.term, opts, out)?;
                out.write_str("</dt>\n")?;
                for def in &item.definitions {
                    out.write_str("<dd>")?;
                    write_inlines(def, opts, out)?;
                    out.write_str("</dd>\n")?;
                }
            }
            out.write_str("</dl>\n")
        }
        Block::FencedDiv { name, children } => {
            if let Some(name) = name {
                out.write_str("<div class=\"")?;
                escape(out, name)?;
                out.write_str("\">\n")?;
            } else {
                out.write_str("<div>\n")?;
            }
            for child in children {
                write_block(child, opts, out)?;
            }
            out.write_str("</div>\n")
        }
        Block::FootnoteDef { label, children } => {
            write!(out, "<div class=\"{}\" id=\"fn-", opts.classes.footnote)?;
            escape(out, label)?;
            out.write_str("\">\n")?;
            for child in children {
                write_block(child, opts, out)?;
            }
            out.write_str("</div>\n")
        }
        Block::Comment { .. } => Ok(()),
        Block::Toc { .. } => Ok(()),
        Block::Include { path, fragment } => {
            out.write_str("<!-- include: ")?;
            escape(out, path)?;
            if let Some(frag) = fragment {
                out.write_char('#')?;
                escape(out, frag)?;
            }
            out.write_str(" -->\n")
        }
    }
}

fn write_inlines(inlines: &[Inline], opts: &HtmlOptions, out: &mut dyn Write) -> fmt::Result {
    for node in inlines {
        write_inline(node, opts, out)?;
    }
    Ok(())
}

fn write_inline(node: &Inline, opts: &HtmlOptions, out: &mut dyn Write) -> fmt::Result {
    match node {
        Inline::Text { value } => escape(out, value),
        Inline::Bold { children } => {
            out.write_str("<strong>")?;
            write_inlines(children, opts, out)?;
            out.write_str("</strong>")
        }
        Inline::Italic { children } => {
            out.write_str("<em>")?;
            write_inlines(children, opts, out)?;
            out.write_str("</em>")
        }
        Inline::BoldItalic { children } => {
            out.write_str("<strong><em>")?;
            write_inlines(children, opts, out)?;
            out.write_str("</em></strong>")
        }
        Inline::Strikethrough { children } => {
            out.write_str("<del>")?;
            write_inlines(children, opts, out)?;
            out.write_str("</del>")
        }
        Inline::Highlight { children } => {
            write!(out, "<mark class=\"{}\">", opts.classes.highlight)?;
            write_inlines(children, opts, out)?;
            out.write_str("</mark>")
        }
        Inline::Superscript { children } => {
            out.write_str("<sup>")?;
            write_inlines(children, opts, out)?;
            out.write_str("</sup>")
        }
        Inline::Subscript { children } => {
            out.write_str("<sub>")?;
            write_inlines(children, opts, out)?;
            out.write_str("</sub>")
        }
        Inline::InlineCode { value } => {
            out.write_str("<code>")?;
            escape(out, value)?;
            out.write_str("</code>")
        }
        Inline::InlineMath { value } => {
            write!(out, "<span class=\"{}\">\\(", opts.classes.math_inline)?;
            escape(out, value)?;
            out.write_str("\\)</span>")
        }
        Inline::Link {
            url,
            title,
            children,
        } => {
            out.write_str("<a href=\"")?;
            escape_href(out, url)?;
            out.write_char('"')?;
            if let Some(title) = title {
                out.write_str(" title=\"")?;
                escape(out, title)?;
                out.write_char('"')?;
            }
            out.write_char('>')?;
            write_inlines(children, opts, out)?;
            out.write_str("</a>")
        }
        Inline::Image {
            url,
            alt,
            title,
            attrs,
        } => {
            out.write_str("<img src=\"")?;
            escape_href(out, url)?;
            out.write_str("\" alt=\"")?;
            escape(out, alt)?;
            out.write_char('"')?;
            if let Some(title) = title {
                out.write_str(" title=\"")?;
                escape(out, title)?;
                out.write_char('"')?;
            }
            for (k, v) in attrs {
                out.write_char(' ')?;
                escape(out, k)?;
                out.write_str("=\"")?;
                escape(out, v)?;
                out.write_char('"')?;
            }
            out.write_char('>')
        }
        Inline::WikiLink {
            target,
            fragment,
            alias,
        } => {
            let lookup_key = match fragment {
                Some(frag) => format!("{target}#{frag}"),
                None => (*target).to_string(),
            };
            let resolved = opts.wikilinks.get(&lookup_key);
            let default_class = if resolved.is_none() && !opts.wikilinks.is_empty() {
                &opts.classes.wikilink_broken
            } else {
                &opts.classes.wikilink
            };
            let class = resolved
                .and_then(|r| r.class.as_deref())
                .unwrap_or(default_class);
            let href = resolved.and_then(|r| r.href.as_deref());
            let display = resolved.and_then(|r| r.display.as_deref());

            write!(out, "<a class=\"{class}\" href=\"")?;
            if let Some(href) = href {
                escape_href(out, href)?;
            } else {
                escape_href(out, target)?;
                if let Some(frag) = fragment {
                    out.write_char('#')?;
                    escape_href(out, frag)?;
                }
            }
            out.write_str("\">")?;
            if let Some(d) = display {
                escape(out, d)?;
            } else if let Some(a) = alias {
                escape(out, a)?;
            } else {
                escape(out, target)?;
            }
            out.write_str("</a>")
        }
        Inline::WikiEmbed {
            target,
            fragment,
            width,
            height,
        } => {
            let lookup_key = match fragment {
                Some(frag) => format!("{target}#{frag}"),
                None => (*target).to_string(),
            };
            if let Some(resolved) = opts.embeds.get(&lookup_key) {
                out.write_str(&resolved.html)
            } else {
                write!(out, "<img class=\"{}\" src=\"", opts.classes.wikiembed)?;
                escape_href(out, target)?;
                if let Some(frag) = fragment {
                    out.write_char('#')?;
                    escape_href(out, frag)?;
                }
                out.write_char('"')?;
                if let Some(w) = width {
                    write!(out, " width=\"{w}\"")?;
                }
                if let Some(h) = height {
                    write!(out, " height=\"{h}\"")?;
                }
                out.write_char('>')
            }
        }
        Inline::BracketedSpan { children, attrs } => {
            out.write_str("<span")?;
            write_span_attrs(attrs, out)?;
            out.write_char('>')?;
            write_inlines(children, opts, out)?;
            out.write_str("</span>")
        }
        Inline::FootnoteRef { label } => {
            write!(
                out,
                "<sup><a class=\"{}\" href=\"#fn-",
                opts.classes.footnote_ref
            )?;
            escape_href(out, label)?;
            out.write_str("\">")?;
            escape(out, label)?;
            out.write_str("</a></sup>")
        }
        Inline::RangeFootnote { label, children } => {
            write!(
                out,
                "<a class=\"{}\" href=\"#fn-",
                opts.classes.footnote_range
            )?;
            escape_href(out, label)?;
            out.write_str("\">")?;
            write_inlines(children, opts, out)?;
            out.write_str("</a>")
        }
        Inline::InlineFootnote { children } => {
            write!(out, "<span class=\"{}\">", opts.classes.footnote_inline)?;
            write_inlines(children, opts, out)?;
            out.write_str("</span>")
        }
        Inline::Citation { keys } => {
            out.write_str("<cite>")?;
            for (i, key) in keys.iter().enumerate() {
                if i > 0 {
                    out.write_str("; ")?;
                }
                if key.suppress_author {
                    out.write_char('-')?;
                }
                escape(out, key.key)?;
                if let Some(loc) = key.locator {
                    out.write_str(", ")?;
                    escape(out, loc)?;
                }
            }
            out.write_str("</cite>")
        }
        Inline::HardBreak {} => out.write_str("<br>\n"),
    }
}

fn write_opt_id(id: &Option<&str>, out: &mut dyn Write) -> fmt::Result {
    if let Some(id) = id {
        out.write_str(" id=\"")?;
        escape(out, id)?;
        out.write_char('"')?;
    }
    Ok(())
}

fn write_table_cell(
    tag: &str,
    align: Option<Alignment>,
    content: &[Inline],
    opts: &HtmlOptions,
    out: &mut dyn Write,
) -> fmt::Result {
    out.write_char('<')?;
    out.write_str(tag)?;
    match align {
        Some(Alignment::Left) => out.write_str(" style=\"text-align: left\"")?,
        Some(Alignment::Center) => out.write_str(" style=\"text-align: center\"")?,
        Some(Alignment::Right) => out.write_str(" style=\"text-align: right\"")?,
        _ => {}
    }
    out.write_char('>')?;
    write_inlines(content, opts, out)?;
    writeln!(out, "</{tag}>")
}

/// Parse `.class`, `#id`, `key=value` attrs into proper HTML attributes.
fn write_span_attrs(attrs: &str, out: &mut dyn Write) -> fmt::Result {
    let mut classes = Vec::new();
    let mut id = None;
    let mut other = Vec::new();

    for part in attrs.split_whitespace() {
        if let Some(cls) = part.strip_prefix('.') {
            classes.push(cls);
        } else if let Some(i) = part.strip_prefix('#') {
            id = Some(i);
        } else if let Some(eq_pos) = part.find('=') {
            other.push((&part[..eq_pos], &part[eq_pos + 1..]));
        }
    }

    if !classes.is_empty() {
        out.write_str(" class=\"")?;
        for (i, cls) in classes.iter().enumerate() {
            if i > 0 {
                out.write_char(' ')?;
            }
            escape(out, cls)?;
        }
        out.write_char('"')?;
    }
    if let Some(id) = id {
        out.write_str(" id=\"")?;
        escape(out, id)?;
        out.write_char('"')?;
    }
    for (k, v) in other {
        out.write_char(' ')?;
        escape(out, k)?;
        out.write_str("=\"")?;
        escape(out, v)?;
        out.write_char('"')?;
    }
    Ok(())
}

fn write_block_children(
    children: &[Block],
    opts: &HtmlOptions,
    out: &mut dyn Write,
) -> fmt::Result {
    if children.len() == 1
        && let Block::Paragraph { content, .. } = &children[0]
    {
        return write_inlines(content, opts, out);
    }
    for child in children {
        write_block(child, opts, out)?;
    }
    Ok(())
}

use crate::ast::block::{Block, Document};
use crate::ast::inline::Inline;

#[derive(Debug)]
pub struct Lint {
    pub message: String,
}

pub fn lint(doc: &Document) -> Vec<Lint> {
    let mut lints = Vec::new();
    let mut prev_level: Option<u8> = None;
    let mut heading_ids: Vec<&str> = Vec::new();
    let mut footnote_defs: Vec<&str> = Vec::new();
    let mut footnote_refs: Vec<&str> = Vec::new();

    collect_blocks(&doc.children, &mut |block| match block {
        Block::Heading { level, id, .. } => {
            if let Some(prev) = prev_level
                && *level > prev + 1
            {
                lints.push(Lint {
                    message: format!("heading level skip: h{prev} -> h{level}"),
                });
            }
            prev_level = Some(*level);
            if let Some(id) = id {
                if heading_ids.contains(id) {
                    lints.push(Lint {
                        message: format!("duplicate heading id: {id}"),
                    });
                }
                heading_ids.push(id);
            }
        }
        Block::FootnoteDef { label, .. } => {
            if footnote_defs.contains(label) {
                lints.push(Lint {
                    message: format!("duplicate footnote definition: {label}"),
                });
            }
            footnote_defs.push(label);
        }
        _ => {}
    });

    collect_inlines_from_blocks(&doc.children, &mut |inline| {
        if let Inline::FootnoteRef { label } = inline {
            footnote_refs.push(label);
        }
    });

    for def in &footnote_defs {
        if !footnote_refs.contains(def) {
            lints.push(Lint {
                message: format!("unused footnote definition: {def}"),
            });
        }
    }
    for r in &footnote_refs {
        if !footnote_defs.contains(r) {
            lints.push(Lint {
                message: format!("undefined footnote reference: {r}"),
            });
        }
    }

    lints
}

fn collect_blocks<'a>(blocks: &'a [Block<'a>], f: &mut dyn FnMut(&'a Block<'a>)) {
    for block in blocks {
        f(block);
        match block {
            Block::Blockquote { children }
            | Block::Admonition { children, .. }
            | Block::FencedDiv { children, .. }
            | Block::FootnoteDef { children, .. } => collect_blocks(children, f),
            Block::UnorderedList { items } => {
                for item in items {
                    collect_blocks(&item.children, f);
                }
            }
            Block::OrderedList { items, .. } => {
                for item in items {
                    collect_blocks(&item.children, f);
                }
            }
            Block::TaskList { items } => {
                for item in items {
                    collect_blocks(&item.children, f);
                }
            }
            _ => {}
        }
    }
}

fn collect_inlines_from_blocks<'a>(blocks: &'a [Block<'a>], f: &mut dyn FnMut(&'a Inline<'a>)) {
    collect_blocks(blocks, &mut |block| match block {
        Block::Heading { content, .. } | Block::Paragraph { content, .. } => {
            collect_inlines(content, f);
        }
        Block::Table {
            headers,
            rows,
            caption,
            ..
        } => {
            for h in headers {
                collect_inlines(h, f);
            }
            for row in rows {
                for cell in row {
                    collect_inlines(cell, f);
                }
            }
            if let Some(cap) = caption {
                collect_inlines(cap, f);
            }
        }
        Block::DefinitionList { items } => {
            for item in items {
                collect_inlines(&item.term, f);
                for def in &item.definitions {
                    collect_inlines(def, f);
                }
            }
        }
        _ => {}
    });
}

fn collect_inlines<'a>(inlines: &'a [Inline<'a>], f: &mut dyn FnMut(&'a Inline<'a>)) {
    for inline in inlines {
        f(inline);
        match inline {
            Inline::Bold { children }
            | Inline::Italic { children }
            | Inline::BoldItalic { children }
            | Inline::Strikethrough { children }
            | Inline::Highlight { children }
            | Inline::Superscript { children }
            | Inline::Subscript { children }
            | Inline::Link { children, .. }
            | Inline::BracketedSpan { children, .. }
            | Inline::RangeFootnote { children, .. }
            | Inline::InlineFootnote { children } => collect_inlines(children, f),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Bump, parse};

    fn lint_src(src: &str) -> Vec<String> {
        let bump = Bump::new();
        let doc = parse(src, &bump);
        lint(&doc).into_iter().map(|l| l.message).collect()
    }

    #[test]
    fn clean_document() {
        assert!(lint_src("# Title\n\n## Section\n").is_empty());
    }

    #[test]
    fn heading_skip() {
        let lints = lint_src("# Title\n\n### Skipped\n");
        assert_eq!(lints, vec!["heading level skip: h1 -> h3"]);
    }

    #[test]
    fn duplicate_id() {
        let lints = lint_src("## A {#x}\n\n## B {#x}\n");
        assert_eq!(lints, vec!["duplicate heading id: x"]);
    }

    #[test]
    fn footnote_undefined() {
        let lints = lint_src("See [^missing].\n");
        assert_eq!(lints, vec!["undefined footnote reference: missing"]);
    }

    #[test]
    fn footnote_unused() {
        let lints = lint_src("[^unused]: Definition\n");
        assert_eq!(lints, vec!["unused footnote definition: unused"]);
    }

    #[test]
    fn footnotes_balanced() {
        assert!(lint_src("See [^a].\n\n[^a]: Note\n").is_empty());
    }

    #[test]
    fn footnote_duplicate_def() {
        let lints = lint_src("See [^a].\n\n[^a]: First\n\n[^a]: Second\n");
        assert_eq!(lints, vec!["duplicate footnote definition: a"]);
    }
}

use crate::ast::block::{Block, Document};
use crate::ast::inline::Inline;
use crate::lint::{collect_blocks, collect_inlines_from_blocks};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExtractedHeading<'a> {
    pub level: u8,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<&'a str>,
    pub text: String,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExtractedWikiLink<'a> {
    pub target: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fragment: Option<&'a str>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub alias: Option<&'a str>,
}

pub fn extract_headings<'a>(doc: &'a Document<'a>) -> Vec<ExtractedHeading<'a>> {
    let mut headings = Vec::new();
    collect_blocks(&doc.children, &mut |block| {
        if let Block::Heading {
            level, id, content, ..
        } = block
        {
            let text: String = content.iter().map(|i| inline_text(i)).collect();
            headings.push(ExtractedHeading {
                level: *level,
                id: *id,
                text,
            });
        }
    });
    headings
}

pub fn extract_wikilinks<'a>(doc: &'a Document<'a>) -> Vec<ExtractedWikiLink<'a>> {
    let mut links = Vec::new();
    collect_inlines_from_blocks(&doc.children, &mut |inline| {
        if let Inline::WikiLink {
            target,
            fragment,
            alias,
        } = inline
        {
            links.push(ExtractedWikiLink {
                target,
                fragment: *fragment,
                alias: *alias,
            });
        }
    });
    links
}

pub fn inline_text(inline: &Inline) -> String {
    match inline {
        Inline::Text { value } => value.to_string(),
        Inline::InlineCode { value } | Inline::InlineMath { value } => value.to_string(),
        Inline::Bold { children }
        | Inline::Italic { children }
        | Inline::BoldItalic { children }
        | Inline::Strikethrough { children }
        | Inline::Highlight { children }
        | Inline::Superscript { children }
        | Inline::Subscript { children }
        | Inline::Link { children, .. }
        | Inline::BracketedSpan { children, .. } => {
            children.iter().map(|i| inline_text(i)).collect()
        }
        _ => String::new(),
    }
}

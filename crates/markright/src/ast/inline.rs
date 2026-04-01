use super::common::CiteKey;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Inline<'a> {
    Text {
        value: &'a str,
    },
    Bold {
        children: Vec<Inline<'a>>,
    },
    Italic {
        children: Vec<Inline<'a>>,
    },
    BoldItalic {
        children: Vec<Inline<'a>>,
    },
    Strikethrough {
        children: Vec<Inline<'a>>,
    },
    Highlight {
        children: Vec<Inline<'a>>,
    },
    Superscript {
        children: Vec<Inline<'a>>,
    },
    Subscript {
        children: Vec<Inline<'a>>,
    },
    InlineCode {
        value: &'a str,
    },
    InlineMath {
        value: &'a str,
    },
    Link {
        url: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        title: Option<&'a str>,
        children: Vec<Inline<'a>>,
    },
    Image {
        url: &'a str,
        alt: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        title: Option<&'a str>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
        attrs: Vec<(&'a str, &'a str)>,
    },
    WikiLink {
        target: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        fragment: Option<&'a str>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        alias: Option<&'a str>,
    },
    WikiEmbed {
        target: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        fragment: Option<&'a str>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        width: Option<u32>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        height: Option<u32>,
    },
    BracketedSpan {
        children: Vec<Inline<'a>>,
        attrs: &'a str,
    },
    FootnoteRef {
        label: &'a str,
    },
    RangeFootnote {
        label: &'a str,
        children: Vec<Inline<'a>>,
    },
    InlineFootnote {
        children: Vec<Inline<'a>>,
    },
    Citation {
        keys: Vec<CiteKey<'a>>,
    },
    HardBreak {},
}

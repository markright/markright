use super::common::{AdmonitionKind, Alignment, TaskState};
use super::inline::Inline;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Document<'a> {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub front_matter: Option<FrontMatter<'a>>,
    pub children: Vec<Block<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct FrontMatter<'a> {
    pub raw: &'a str,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Block<'a> {
    Heading {
        level: u8,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        id: Option<&'a str>,
        content: Vec<Inline<'a>>,
    },
    Paragraph {
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        id: Option<&'a str>,
        content: Vec<Inline<'a>>,
    },
    CodeBlock {
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        lang: Option<&'a str>,
        body: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        id: Option<&'a str>,
    },
    DiagramBlock {
        lang: &'a str,
        body: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        id: Option<&'a str>,
    },
    MathBlock {
        body: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        id: Option<&'a str>,
    },
    ThematicBreak {},
    Blockquote {
        children: Vec<Block<'a>>,
    },
    UnorderedList {
        items: Vec<ListItem<'a>>,
    },
    OrderedList {
        start: u32,
        items: Vec<ListItem<'a>>,
    },
    TaskList {
        items: Vec<TaskItem<'a>>,
    },
    Table {
        headers: Vec<Vec<Inline<'a>>>,
        alignments: Vec<Alignment>,
        rows: Vec<Vec<Vec<Inline<'a>>>>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        caption: Option<Vec<Inline<'a>>>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        id: Option<&'a str>,
    },
    Admonition {
        kind: AdmonitionKind,
        foldable: bool,
        children: Vec<Block<'a>>,
    },
    DefinitionList {
        items: Vec<DefItem<'a>>,
    },
    FencedDiv {
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        name: Option<&'a str>,
        children: Vec<Block<'a>>,
    },
    FootnoteDef {
        label: &'a str,
        children: Vec<Block<'a>>,
    },
    Comment {
        body: &'a str,
    },
    Toc {
        min_level: u8,
        max_level: u8,
    },
    Include {
        path: &'a str,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        fragment: Option<&'a str>,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ListItem<'a> {
    pub children: Vec<Block<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TaskItem<'a> {
    pub state: TaskState,
    pub children: Vec<Block<'a>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct DefItem<'a> {
    pub term: Vec<Inline<'a>>,
    pub definitions: Vec<Vec<Inline<'a>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum TaskState {
    Open,
    Done,
    Active,
    Review,
    Cancelled,
    Blocked,
    Deferred,
    Question,
}

impl TaskState {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            ' ' => Some(Self::Open),
            'x' => Some(Self::Done),
            '~' => Some(Self::Active),
            '@' => Some(Self::Review),
            '-' => Some(Self::Cancelled),
            '!' => Some(Self::Blocked),
            '>' => Some(Self::Deferred),
            '?' => Some(Self::Question),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Open => ' ',
            Self::Done => 'x',
            Self::Active => '~',
            Self::Review => '@',
            Self::Cancelled => '-',
            Self::Blocked => '!',
            Self::Deferred => '>',
            Self::Question => '?',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum AdmonitionKind {
    Note,
    Tip,
    Warning,
    Important,
    Caution,
}

impl AdmonitionKind {
    pub fn from_prefix(c: char) -> Option<Self> {
        match c {
            'N' => Some(Self::Note),
            'T' => Some(Self::Tip),
            'W' => Some(Self::Warning),
            '!' => Some(Self::Important),
            'X' => Some(Self::Caution),
            _ => None,
        }
    }

    pub fn prefix(self) -> char {
        match self {
            Self::Note => 'N',
            Self::Tip => 'T',
            Self::Warning => 'W',
            Self::Important => '!',
            Self::Caution => 'X',
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CiteKey<'a> {
    pub key: &'a str,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub locator: Option<&'a str>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub suppress_author: bool,
}

use super::parse_recursive;
use crate::ast::inline::Inline;

/// Try to parse [^label] footnote reference at `pos`.
pub fn try_parse_ref<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    if !input[pos..].starts_with("[^") {
        return None;
    }

    // Check for range footnote: [^label>text]
    let close = input[pos + 2..].find(']')? + pos + 2;
    let inner = &input[pos + 2..close];

    if let Some(arrow) = inner.find('>') {
        // Range footnote: [^label>text]
        let label = &inner[..arrow];
        let text = &inner[arrow + 1..];
        let children = parse_recursive(text);
        return Some((Inline::RangeFootnote { label, children }, close + 1));
    }

    // Point footnote: [^label]
    if inner.is_empty()
        || !inner
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    {
        return None;
    }

    Some((Inline::FootnoteRef { label: inner }, close + 1))
}

/// Try to parse ^[inline footnote] at `pos`.
pub fn try_parse_inline<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    if !input[pos..].starts_with("^[") {
        return None;
    }

    let close = input[pos + 2..].find(']')? + pos + 2;
    let inner = &input[pos + 2..close];
    let children = parse_recursive(inner);

    Some((Inline::InlineFootnote { children }, close + 1))
}

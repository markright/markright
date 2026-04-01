use crate::ast::common::CiteKey;
use crate::ast::inline::Inline;

/// Try to parse [@key] or [@key1; @key2, p. 42] at `pos`.
pub fn try_parse<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    if !input[pos..].starts_with("[@") {
        return None;
    }

    let close = input[pos + 1..].find(']')? + pos + 1;
    let inner = &input[pos + 1..close]; // includes the leading @

    let parts: Vec<&str> = inner.split(';').collect();
    let mut keys = Vec::new();

    for part in parts {
        let trimmed = part.trim();
        let (suppress_author, key_part) = if let Some(rest) = trimmed.strip_prefix("-@") {
            (true, rest)
        } else if let Some(rest) = trimmed.strip_prefix('@') {
            (false, rest)
        } else {
            return None;
        };

        let (key, locator) = if let Some(comma) = key_part.find(',') {
            (&key_part[..comma], Some(key_part[comma + 1..].trim()))
        } else {
            (key_part, None)
        };

        keys.push(CiteKey {
            key,
            locator,
            suppress_author,
        });
    }

    if keys.is_empty() {
        return None;
    }

    Some((Inline::Citation { keys }, close + 1))
}

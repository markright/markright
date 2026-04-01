use super::parse_recursive;
use crate::ast::inline::Inline;

/// Try to parse emphasis starting with * at `pos`.
/// Handles: ***bold italic***, **bold**, *italic*
pub fn try_parse<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    let rest = &input[pos..];

    if rest.starts_with("***")
        && let Some(end) = find_closing(input, pos + 3, "***")
    {
        let children = parse_recursive(&input[pos + 3..end]);
        return Some((Inline::BoldItalic { children }, end + 3));
    }

    if rest.starts_with("**")
        && let Some(end) = find_closing(input, pos + 2, "**")
    {
        let children = parse_recursive(&input[pos + 2..end]);
        return Some((Inline::Bold { children }, end + 2));
    }

    if rest.starts_with('*')
        && !rest.starts_with("**")
        && let Some(end) = find_closing(input, pos + 1, "*")
    {
        let children = parse_recursive(&input[pos + 1..end]);
        return Some((Inline::Italic { children }, end + 1));
    }

    None
}

/// Try to parse ~~strikethrough~~ at `pos`.
pub fn try_parse_strikethrough<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    try_parse_wrapped(input, pos, "~~", |children| Inline::Strikethrough {
        children,
    })
}

/// Try to parse ==highlight== at `pos`.
pub fn try_parse_highlight<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    try_parse_wrapped(input, pos, "==", |children| Inline::Highlight { children })
}

fn try_parse_wrapped<'a, F>(
    input: &'a str,
    pos: usize,
    delim: &str,
    ctor: F,
) -> Option<(Inline<'a>, usize)>
where
    F: FnOnce(Vec<Inline<'a>>) -> Inline<'a>,
{
    if !input[pos..].starts_with(delim) {
        return None;
    }
    let dlen = delim.len();
    let end = find_closing(input, pos + dlen, delim)?;
    let children = parse_recursive(&input[pos + dlen..end]);
    Some((ctor(children), end + dlen))
}

/// Find the position of a closing delimiter, skipping escaped characters.
fn find_closing(input: &str, from: usize, delim: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    let delim_bytes = delim.as_bytes();
    let mut i = from;

    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if i + delim_bytes.len() <= bytes.len()
            && &bytes[i..i + delim_bytes.len()] == delim_bytes
            && i > from
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

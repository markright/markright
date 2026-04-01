use super::parse_recursive;
use crate::ast::inline::Inline;

/// Try to parse ^superscript^ at `pos`.
pub fn try_parse_sup<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    // Don't match ^[ (inline footnote)
    if input.as_bytes().get(pos + 1) == Some(&b'[') {
        return None;
    }
    try_parse_delimited(input, pos, b'^', |children| Inline::Superscript {
        children,
    })
}

/// Try to parse ~subscript~ at `pos`.
pub fn try_parse_sub<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    // Don't match ~~ (strikethrough)
    if input.as_bytes().get(pos + 1) == Some(&b'~') {
        return None;
    }
    try_parse_delimited(input, pos, b'~', |children| Inline::Subscript { children })
}

fn try_parse_delimited<'a, F>(
    input: &'a str,
    pos: usize,
    delim: u8,
    ctor: F,
) -> Option<(Inline<'a>, usize)>
where
    F: FnOnce(Vec<Inline<'a>>) -> Inline<'a>,
{
    if input.as_bytes()[pos] != delim {
        return None;
    }

    let rest = &input[pos + 1..];
    let close = rest.find(delim as char)?;
    if close == 0 {
        return None;
    }

    let inner = &rest[..close];
    if inner.contains(' ') || inner.contains('\n') {
        return None;
    }

    let children = parse_recursive(inner);
    Some((ctor(children), pos + 1 + close + 1))
}

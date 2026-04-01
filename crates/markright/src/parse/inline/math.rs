use crate::ast::inline::Inline;

/// Try to parse $inline math$ at `pos`.
pub fn try_parse<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    if input.as_bytes()[pos] != b'$' {
        return None;
    }
    // Don't match $$ (that's a block math delimiter)
    if input.as_bytes().get(pos + 1) == Some(&b'$') {
        return None;
    }

    let rest = &input[pos + 1..];
    let close = rest.find('$')?;

    // Don't match empty: $$
    if close == 0 {
        return None;
    }

    let value = &rest[..close];

    // Don't allow newlines in inline math
    if value.contains('\n') {
        return None;
    }

    Some((Inline::InlineMath { value }, pos + 1 + close + 1))
}

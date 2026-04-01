use crate::ast::inline::Inline;

/// Try to parse `inline code` at `pos`.
pub fn try_parse<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    let bytes = input.as_bytes();

    // Count opening backticks
    let mut ticks = 0;
    let mut i = pos;
    while i < bytes.len() && bytes[i] == b'`' {
        ticks += 1;
        i += 1;
    }
    if ticks == 0 {
        return None;
    }

    // Find matching closing backticks
    let needle = &input[pos..pos + ticks];
    let close = input[i..].find(needle)?;
    let end = i + close;

    let value = &input[i..end];

    Some((Inline::InlineCode { value }, end + ticks))
}

use crate::ast::inline::Inline;

/// Try to parse [[target]], [[target|alias]], [[target#fragment]] at `pos`.
pub fn try_parse_link<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    if !input[pos..].starts_with("[[") {
        return None;
    }

    let close = input[pos + 2..].find("]]")? + pos + 2;
    let inner = &input[pos + 2..close];

    let (target_part, alias) = split_on_char(inner, '|');
    let (target, fragment) = split_on_char(target_part, '#');

    Some((
        Inline::WikiLink {
            target,
            fragment,
            alias,
        },
        close + 2,
    ))
}

/// Try to parse ![[target]], ![[target|640]], ![[target|640x480]] at `pos`.
pub fn try_parse_embed<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    if !input[pos..].starts_with("![[") {
        return None;
    }

    let close = input[pos + 3..].find("]]")? + pos + 3;
    let inner = &input[pos + 3..close];

    let (target_part, size_part) = split_on_char(inner, '|');
    let (target, fragment) = split_on_char(target_part, '#');

    let (width, height) = if let Some(size) = size_part {
        if let Some(x_pos) = size.find('x') {
            let w: u32 = size[..x_pos].parse().ok()?;
            let h: u32 = size[x_pos + 1..].parse().ok()?;
            (Some(w), Some(h))
        } else {
            let w: u32 = size.parse().ok()?;
            (Some(w), None)
        }
    } else {
        (None, None)
    };

    Some((
        Inline::WikiEmbed {
            target,
            fragment,
            width,
            height,
        },
        close + 2,
    ))
}

/// Split a string on the first occurrence of `delim`, returning (before, Some(after)) or (input, None).
fn split_on_char(s: &str, delim: char) -> (&str, Option<&str>) {
    match s.find(delim) {
        Some(pos) => (&s[..pos], Some(&s[pos + 1..])),
        None => (s, None),
    }
}

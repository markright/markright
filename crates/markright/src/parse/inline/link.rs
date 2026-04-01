use super::parse_recursive;
use crate::ast::inline::Inline;

/// Try to parse [text](url), [text](url "title"), or [text]{attrs} at `pos`.
pub fn try_parse_link_or_span<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    let close_bracket = find_matching(input, pos, b'[', b']')?;
    let text = &input[pos + 1..close_bracket];

    let after = close_bracket + 1;
    if after >= input.len() {
        return None;
    }

    match input.as_bytes()[after] {
        b'(' => {
            let close_paren = find_matching(input, after, b'(', b')')?;
            let (url, title) = split_url_title(&input[after + 1..close_paren]);
            let children = parse_recursive(text);
            Some((
                Inline::Link {
                    url,
                    title,
                    children,
                },
                close_paren + 1,
            ))
        }
        b'{' => {
            let close_brace = input[after..].find('}')? + after;
            let attrs = &input[after + 1..close_brace];
            let children = parse_recursive(text);
            Some((Inline::BracketedSpan { children, attrs }, close_brace + 1))
        }
        _ => None,
    }
}

/// Try to parse ![alt](url) possibly with {attrs} at `pos`.
pub fn try_parse_image<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    if input.as_bytes().get(pos + 1) != Some(&b'[') {
        return None;
    }
    if input.as_bytes().get(pos + 2) == Some(&b'[') {
        return None;
    }

    let close_bracket = find_matching(input, pos + 1, b'[', b']')?;
    let alt = &input[pos + 2..close_bracket];

    if input.as_bytes().get(close_bracket + 1) != Some(&b'(') {
        return None;
    }

    let close_paren = find_matching(input, close_bracket + 1, b'(', b')')?;
    let (url, title) = split_url_title(&input[close_bracket + 2..close_paren]);

    let mut end = close_paren + 1;
    let mut attrs = Vec::new();

    if input.as_bytes().get(end) == Some(&b'{')
        && let Some(close_brace) = input[end..].find('}')
    {
        let attr_str = &input[end + 1..end + close_brace];
        attrs = parse_attrs(attr_str);
        end = end + close_brace + 1;
    }

    Some((
        Inline::Image {
            url,
            alt,
            title,
            attrs,
        },
        end,
    ))
}

/// Try to parse a bare URL autolink at `pos`.
pub fn try_parse_autolink<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    let rest = &input[pos..];
    if !rest.starts_with("https://") && !rest.starts_with("http://") {
        return None;
    }

    let end = rest
        .find(|c: char| c.is_whitespace() || matches!(c, ')' | ']' | '>' | '<'))
        .unwrap_or(rest.len());

    let url = &input[pos..pos + end];
    Some((
        Inline::Link {
            url,
            title: None,
            children: vec![Inline::Text { value: url }],
        },
        pos + end,
    ))
}

/// Split "url" or "url \"title\"" into (url, Option<title>).
fn split_url_title(url_part: &str) -> (&str, Option<&str>) {
    if let Some(quote_start) = url_part.rfind(" \"")
        && url_part.ends_with('"')
    {
        return (
            &url_part[..quote_start],
            Some(&url_part[quote_start + 2..url_part.len() - 1]),
        );
    }
    (url_part, None)
}

fn find_matching(input: &str, start: usize, open: u8, close: u8) -> Option<usize> {
    let bytes = input.as_bytes();
    let mut depth = 0;
    let mut i = start;

    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == open {
            depth += 1;
        } else if bytes[i] == close {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn parse_attrs(s: &str) -> Vec<(&str, &str)> {
    let mut attrs = Vec::new();
    for part in s.split_whitespace() {
        if let Some(eq_pos) = part.find('=') {
            attrs.push((&part[..eq_pos], &part[eq_pos + 1..]));
        }
    }
    attrs
}

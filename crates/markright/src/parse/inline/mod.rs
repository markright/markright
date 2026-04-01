pub mod citation;
pub mod code;
pub mod emphasis;
pub mod footnote;
pub mod link;
pub mod math;
pub mod super_sub;
pub mod wikilink;

use crate::ast::inline::Inline;

/// Parse inline content.
pub fn parse<'a>(input: &'a str) -> Vec<Inline<'a>> {
    if input.is_empty() {
        return vec![];
    }
    parse_recursive(input)
}

fn parse_recursive<'a>(input: &'a str) -> Vec<Inline<'a>> {
    let mut nodes: Vec<Inline<'a>> = Vec::new();
    let mut pos = 0;
    let bytes = input.as_bytes();

    while pos < bytes.len() {
        if let Some((node, end)) = try_parse_at(input, pos) {
            nodes.push(node);
            pos = end;
            continue;
        }

        let text_start = pos;
        pos += 1;
        while pos < bytes.len() && !is_inline_marker_at(bytes, pos) {
            pos += 1;
        }
        nodes.push(Inline::Text {
            value: &input[text_start..pos],
        });
    }

    nodes
}

fn try_parse_at<'a>(input: &'a str, pos: usize) -> Option<(Inline<'a>, usize)> {
    let bytes = input.as_bytes();
    let b = bytes[pos];

    match b {
        b'\\' if pos + 1 < bytes.len() => {
            if bytes[pos + 1] == b'\n' {
                Some((Inline::HardBreak {}, pos + 2))
            } else {
                Some((
                    Inline::Text {
                        value: &input[pos + 1..pos + 2],
                    },
                    pos + 2,
                ))
            }
        }
        b'`' => code::try_parse(input, pos),
        b'$' if bytes.get(pos + 1) != Some(&b'$') => math::try_parse(input, pos),
        b'*' => emphasis::try_parse(input, pos),
        b'~' if bytes.get(pos + 1) == Some(&b'~') => emphasis::try_parse_strikethrough(input, pos),
        b'~' if bytes.get(pos + 1) != Some(&b'~') => super_sub::try_parse_sub(input, pos),
        b'=' if bytes.get(pos + 1) == Some(&b'=') => emphasis::try_parse_highlight(input, pos),
        b'^' if bytes.get(pos + 1) == Some(&b'[') => footnote::try_parse_inline(input, pos),
        b'^' => super_sub::try_parse_sup(input, pos),
        b'!' if bytes.get(pos + 1) == Some(&b'[') && bytes.get(pos + 2) == Some(&b'[') => {
            wikilink::try_parse_embed(input, pos)
        }
        b'!' if bytes.get(pos + 1) == Some(&b'[') => link::try_parse_image(input, pos),
        b'[' if bytes.get(pos + 1) == Some(&b'[') => wikilink::try_parse_link(input, pos),
        b'[' if bytes.get(pos + 1) == Some(&b'^') => footnote::try_parse_ref(input, pos),
        b'[' if bytes.get(pos + 1) == Some(&b'@') => citation::try_parse(input, pos),
        b'[' => link::try_parse_link_or_span(input, pos),
        b'h' if input[pos..].starts_with("https://") || input[pos..].starts_with("http://") => {
            link::try_parse_autolink(input, pos)
        }
        _ => None,
    }
}

fn is_inline_marker_at(input: &[u8], pos: usize) -> bool {
    let b = input[pos];
    match b {
        b'\\' | b'`' | b'$' | b'*' | b'[' | b'!' | b'^' | b'~' | b'=' => true,
        b'h' => {
            let rest = &input[pos..];
            rest.starts_with(b"https://") || rest.starts_with(b"http://")
        }
        _ => false,
    }
}

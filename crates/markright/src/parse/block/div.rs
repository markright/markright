use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_div(&mut self) -> Option<Block<'a>> {
        let line = self.current_line();
        let fence_len = line.bytes().take_while(|&b| b == b':').count();
        if fence_len < 3 {
            return None;
        }

        let info = line[fence_len..].trim();
        let name = if info.is_empty() { None } else { Some(info) };

        self.advance_line();
        let body = self.consume_fenced_body(|candidate| {
            let close_len = candidate.bytes().take_while(|&b| b == b':').count();
            close_len >= fence_len && candidate.len() == close_len
        });
        let inner_lines: Vec<&'a str> = if body.is_empty() {
            Vec::new()
        } else {
            body.lines().collect()
        };

        let children = self.parse_sub(&inner_lines);
        Some(Block::FencedDiv { name, children })
    }
}

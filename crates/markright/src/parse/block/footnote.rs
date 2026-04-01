use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_footnote(&mut self) -> Option<Block<'a>> {
        let line = self.current_line();
        if !line.starts_with("[^") {
            return None;
        }

        let close = line.find("]:")?;
        let label = &line[2..close];
        if label.is_empty() {
            return None;
        }

        let first_content = line[close + 2..].trim_start();
        let mut content_lines: Vec<&'a str> = vec![first_content];

        self.advance_line();
        while !self.at_end() && self.current_line().starts_with("    ") {
            content_lines.push(&self.current_line()[4..]);
            self.advance_line();
        }

        let children = self.parse_sub(&content_lines);
        Some(Block::FootnoteDef { label, children })
    }
}

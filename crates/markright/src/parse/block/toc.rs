use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_toc(&mut self) -> Option<Block<'a>> {
        let trimmed = self.current_line().trim();

        if trimmed == "[TOC]" {
            self.advance_line();
            return Some(Block::Toc {
                min_level: 1,
                max_level: 6,
            });
        }

        if trimmed.starts_with("[TOC:") && trimmed.ends_with(']') {
            let range = &trimmed[5..trimmed.len() - 1];
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() == 2 {
                let min: u8 = parts[0].parse().ok()?;
                let max: u8 = parts[1].parse().ok()?;
                if min >= 1 && max <= 6 && min <= max {
                    self.advance_line();
                    return Some(Block::Toc {
                        min_level: min,
                        max_level: max,
                    });
                }
            }
        }

        None
    }
}

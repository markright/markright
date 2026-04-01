use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_math_block(&mut self) -> Option<Block<'a>> {
        if self.current_line().trim() != "$$" {
            return None;
        }

        self.advance_line();
        let body = self.consume_fenced_body(|candidate| candidate == "$$");

        Some(Block::MathBlock { body, id: None })
    }
}

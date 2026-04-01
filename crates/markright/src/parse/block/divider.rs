use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_divider(&mut self) -> Option<Block<'a>> {
        if self.current_line().trim() != "---" {
            return None;
        }
        self.advance_line();
        Some(Block::ThematicBreak {})
    }
}

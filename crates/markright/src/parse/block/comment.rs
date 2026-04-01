use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_comment(&mut self) -> Option<Block<'a>> {
        let line = self.current_line();

        // Single-line: %% text %%
        if line.starts_with("%%") && line.len() >= 4 && line.ends_with("%%") {
            let inner = line[2..line.len() - 2].trim();
            self.advance_line();
            return Some(Block::Comment { body: inner });
        }

        // Multi-line: %% on its own line
        if line.trim() == "%%" {
            self.advance_line();
            let body = self.consume_fenced_body(|candidate| candidate == "%%");
            return Some(Block::Comment { body });
        }

        None
    }
}

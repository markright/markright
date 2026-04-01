use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_blockquote(&mut self) -> Option<Block<'a>> {
        let line = self.current_line();
        if !line.starts_with("> ") && line != ">" {
            return None;
        }

        let mut inner = String::new();
        while !self.at_end() {
            let l = self.current_line();
            if let Some(rest) = l.strip_prefix("> ") {
                inner.push_str(rest);
                inner.push('\n');
            } else if l == ">" {
                inner.push('\n');
            } else {
                break;
            }
            self.advance_line();
        }

        let src = self.bump.alloc_str(inner);
        let mut sub = Parser::new(src, self.bump);
        let children = sub.parse_blocks();

        Some(Block::Blockquote { children })
    }
}

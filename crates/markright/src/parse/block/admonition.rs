use crate::ast::block::Block;
use crate::ast::common::AdmonitionKind;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_admonition(&mut self) -> Option<Block<'a>> {
        let line = self.current_line();
        if line.len() < 2 {
            return None;
        }

        let prefix_char = line.chars().next()?;
        let kind = AdmonitionKind::from_prefix(prefix_char)?;

        if line.as_bytes().get(1) != Some(&b'>') {
            return None;
        }

        let after_prefix = &line[2..];
        let (foldable, first_content) = if let Some(rest) = after_prefix.strip_prefix("- ") {
            (true, rest)
        } else if after_prefix == "-" {
            (true, "")
        } else if let Some(rest) = after_prefix.strip_prefix(' ') {
            (false, rest)
        } else if after_prefix.is_empty() {
            (false, "")
        } else {
            return None;
        };

        let prefix_bytes: [u8; 2] = [line.as_bytes()[0], b'>'];

        let mut inner = String::new();
        if !first_content.is_empty() {
            inner.push_str(first_content);
            inner.push('\n');
        }

        self.advance_line();
        while !self.at_end() {
            let l = self.current_line();
            if l.as_bytes().get(..2) == Some(&prefix_bytes) {
                let rest = &l[2..];
                if let Some(stripped) = rest.strip_prefix(' ') {
                    inner.push_str(stripped);
                } else {
                    inner.push_str(rest);
                }
                inner.push('\n');
            } else {
                break;
            }
            self.advance_line();
        }

        let src = self.bump.alloc_str(inner);
        let mut sub = Parser::new(src, self.bump);
        let children = sub.parse_blocks();

        Some(Block::Admonition {
            kind,
            foldable,
            children,
        })
    }
}

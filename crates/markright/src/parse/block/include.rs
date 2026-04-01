use crate::ast::block::Block;
use crate::parse::Parser;

impl<'a> Parser<'a> {
    pub(crate) fn try_include(&mut self) -> Option<Block<'a>> {
        let trimmed = self.current_line().trim();
        if !trimmed.starts_with("{{") || !trimmed.ends_with("}}") {
            return None;
        }

        let inner = trimmed[2..trimmed.len() - 2].trim();
        if inner.is_empty() {
            return None;
        }

        let (path, fragment) = if let Some(hash_pos) = inner.find('#') {
            (
                inner[..hash_pos].trim_end(),
                Some(inner[hash_pos + 1..].trim_start()),
            )
        } else {
            (inner, None)
        };

        self.advance_line();
        Some(Block::Include { path, fragment })
    }
}

use super::extract_trailing_id;
use crate::ast::block::Block;
use crate::parse::Parser;
use crate::parse::inline;

impl<'a> Parser<'a> {
    pub(crate) fn try_heading(&mut self) -> Option<Block<'a>> {
        let line = self.current_line();
        let hashes = line.bytes().take_while(|&b| b == b'#').count();
        if hashes == 0 || hashes > 6 {
            return None;
        }

        let rest = &line[hashes..];
        if !rest.starts_with(' ') {
            return None;
        }

        let text = rest[1..].trim_end();
        let (text, id) = extract_trailing_id(text);
        let content = inline::parse(text);

        self.advance_line();
        Some(Block::Heading {
            level: hashes as u8,
            id,
            content,
        })
    }
}

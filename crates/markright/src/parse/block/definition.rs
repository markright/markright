use crate::ast::block::{Block, DefItem};
use crate::parse::Parser;
use crate::parse::inline;

impl<'a> Parser<'a> {
    pub(crate) fn try_definition_list(&mut self) -> Option<Block<'a>> {
        let first_char = self.current_line().as_bytes().first()?;
        if matches!(
            first_char,
            b'#' | b'>' | b'-' | b'`' | b'$' | b'%' | b':' | b'|'
        ) {
            return None;
        }

        let next_start = self.next_line_start()?;
        if !self.line_at(next_start).starts_with(": ") {
            return None;
        }

        let mut items = Vec::new();
        while !self.at_end() {
            if !self
                .next_line_start()
                .is_some_and(|next| self.line_at(next).starts_with(": "))
            {
                break;
            }

            let term = inline::parse(self.current_line().trim());
            self.advance_line();

            let mut definitions = Vec::new();
            while !self.at_end() && self.current_line().starts_with(": ") {
                definitions.push(inline::parse(&self.current_line()[2..]));
                self.advance_line();
            }

            items.push(DefItem { term, definitions });

            self.skip_blank_lines();
        }

        Some(Block::DefinitionList { items })
    }
}

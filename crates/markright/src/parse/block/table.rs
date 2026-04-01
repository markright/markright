use super::extract_trailing_id;
use crate::ast::block::Block;
use crate::ast::common::Alignment;
use crate::parse::Parser;
use crate::parse::inline;

fn parse_separator(line: &str) -> Option<Vec<Alignment>> {
    let cells = split_row(line);
    let mut alignments = Vec::new();

    for cell in &cells {
        let trimmed = cell.trim();
        if trimmed.is_empty() {
            continue;
        }
        let stripped = trimmed.trim_matches(|c: char| c == ':' || c == '-' || c == ' ');
        if !stripped.is_empty() {
            return None;
        }
        if !trimmed.contains('-') {
            return None;
        }

        let left = trimmed.starts_with(':');
        let right = trimmed.ends_with(':');
        alignments.push(match (left, right) {
            (true, true) => Alignment::Center,
            (true, false) => Alignment::Left,
            (false, true) => Alignment::Right,
            (false, false) => Alignment::None,
        });
    }

    if alignments.is_empty() {
        return None;
    }
    Some(alignments)
}

fn split_row(line: &str) -> Vec<&str> {
    let mut s = line.trim();
    if s.starts_with('|') {
        s = &s[1..];
    }
    if s.ends_with('|') {
        s = &s[..s.len() - 1];
    }
    s.split('|').collect()
}

impl<'a> Parser<'a> {
    pub(crate) fn try_table(&mut self) -> Option<Block<'a>> {
        let header_line = self.current_line();
        if !header_line.contains('|') {
            return None;
        }

        let sep_start = self.next_line_start()?;
        let sep_line = self.line_at(sep_start);
        if !sep_line.contains('|') {
            return None;
        }

        let alignments = parse_separator(sep_line)?;

        let headers: Vec<Vec<_>> = split_row(header_line)
            .iter()
            .map(|c| inline::parse(c.trim()))
            .collect();

        self.advance_line(); // past header
        self.advance_line(); // past separator

        let mut rows = Vec::new();
        while !self.at_end() && self.current_line().contains('|') {
            let row: Vec<Vec<_>> = split_row(self.current_line())
                .iter()
                .map(|c| inline::parse(c.trim()))
                .collect();
            rows.push(row);
            self.advance_line();
        }

        let mut caption = None;
        let mut id = None;
        let saved = self.pos;

        self.skip_blank_lines();
        if !self.at_end() && self.current_line().starts_with("Table: ") {
            let cap_text = &self.current_line()[7..];
            let (cap_text, cap_id) = extract_trailing_id(cap_text);
            id = cap_id;
            caption = Some(inline::parse(cap_text));
            self.advance_line();
        } else {
            self.pos = saved;
        }

        Some(Block::Table {
            headers,
            alignments,
            rows,
            caption,
            id,
        })
    }
}

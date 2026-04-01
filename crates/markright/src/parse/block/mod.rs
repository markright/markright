mod admonition;
mod code;
mod comment;
mod definition;
mod div;
mod divider;
mod footnote;
mod heading;
mod include;
mod list;
mod math;
mod quote;
mod table;
mod toc;

use crate::ast::block::Block;
use crate::parse::Parser;
use crate::parse::inline;

/// Strip a trailing `{#id}` if present.
pub(crate) fn extract_trailing_id(text: &str) -> (&str, Option<&str>) {
    if let Some(brace_start) = text.rfind("{#")
        && text.ends_with('}')
    {
        let id = &text[brace_start + 2..text.len() - 1];
        let clean = text[..brace_start].trim_end();
        return (clean, Some(id));
    }
    (text, None)
}

impl<'a> Parser<'a> {
    /// Parse all blocks. Block dispatch doubles as paragraph interruption --
    /// all try_* parsers are position-safe on failure.
    pub fn parse_blocks(&mut self) -> Vec<Block<'a>> {
        let mut blocks = Vec::new();
        let mut para_start: Option<usize> = None;

        while !self.at_end() {
            if self.current_line().trim().is_empty() {
                if let Some(start) = para_start.take() {
                    blocks.push(self.finish_paragraph(start, self.pos));
                }
                self.skip_blank_lines();
                continue;
            }

            let saved = self.pos;
            if let Some(block) = self.try_block() {
                if let Some(start) = para_start.take() {
                    blocks.push(self.finish_paragraph(start, saved));
                }
                blocks.push(block);
                continue;
            }

            if para_start.is_none() {
                para_start = Some(self.pos);
            }
            self.advance_line();
        }

        if let Some(start) = para_start {
            blocks.push(self.finish_paragraph(start, self.pos));
        }

        blocks
    }

    /// First-byte dispatch: route to the right parser in one match.
    fn try_block(&mut self) -> Option<Block<'a>> {
        let line = self.current_line();
        let first = *line.as_bytes().first()?;

        let result = match first {
            b'%' => self.try_comment(),
            b'$' => self.try_math_block(),
            b'`' => self.try_code_block(),
            b':' => self.try_div(),
            b'-' => self
                .try_divider()
                .or_else(|| self.try_task_list())
                .or_else(|| self.try_unordered_list()),
            b'#' => self.try_heading().or_else(|| self.try_ordered_list()),
            b'N' | b'T' | b'W' | b'!' | b'X' => self.try_admonition(),
            b'>' => self.try_blockquote(),
            b'[' => self.try_toc().or_else(|| self.try_footnote()),
            b'0'..=b'9' => self.try_ordered_list(),
            b'{' => self.try_include(),
            b' ' | b'\t' => self.try_indented_block(),
            _ => None,
        };

        result
            .or_else(|| self.try_table())
            .or_else(|| self.try_definition_list())
    }

    fn try_indented_block(&mut self) -> Option<Block<'a>> {
        let trimmed = self.current_line().trim();
        match trimmed.as_bytes().first()? {
            b'$' => self.try_math_block(),
            b'-' => self.try_divider(),
            b'%' => self.try_comment(),
            b'[' => self.try_toc(),
            b'{' => self.try_include(),
            _ => None,
        }
    }

    fn finish_paragraph(&self, start: usize, end: usize) -> Block<'a> {
        let raw = self.src[start..end].trim_end_matches('\n');
        let text = if !raw.contains('\n') {
            raw.trim()
        } else {
            let joined = raw.lines().map(|l| l.trim()).collect::<Vec<_>>().join(" ");
            self.bump.alloc_str(joined)
        };
        let (text, id) = extract_trailing_id(text);
        let content = inline::parse(text);
        Block::Paragraph { id, content }
    }
}

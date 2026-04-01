use crate::ast::block::{Block, ListItem, TaskItem};
use crate::ast::common::TaskState;
use crate::parse::Parser;

fn parse_task_line(line: &str) -> Option<(TaskState, &str)> {
    if !line.starts_with("- [") || line.len() < 6 {
        return None;
    }
    if line.as_bytes()[4] != b']' || line.as_bytes()[5] != b' ' {
        return None;
    }
    let state = TaskState::from_char(line.as_bytes()[3] as char)?;
    Some((state, &line[6..]))
}

fn parse_ordered_line(line: &str) -> Option<(u32, &str)> {
    if let Some(rest) = line.strip_prefix("#. ") {
        return Some((1, rest));
    }
    let dot_pos = line.find(". ")?;
    let num: u32 = line[..dot_pos].parse().ok()?;
    Some((num, &line[dot_pos + 2..]))
}

impl<'a> Parser<'a> {
    /// Collect indented continuation lines for a list item.
    fn collect_continuation(&mut self, indent: &str) -> Vec<&'a str> {
        let mut lines = Vec::new();
        while !self.at_end() {
            let next = self.current_line();
            if next.trim().is_empty() {
                lines.push("");
                self.advance_line();
            } else if let Some(rest) = next.strip_prefix(indent) {
                lines.push(rest);
                self.advance_line();
            } else {
                break;
            }
        }
        lines
    }

    pub(crate) fn try_task_list(&mut self) -> Option<Block<'a>> {
        parse_task_line(self.current_line())?;

        let mut items = Vec::new();
        while !self.at_end() {
            let line = self.current_line();
            let Some((state, rest)) = parse_task_line(line) else {
                break;
            };

            self.advance_line();
            let mut item_lines = vec![rest];
            item_lines.extend(self.collect_continuation("  "));

            let children = self.parse_sub(&item_lines);
            items.push(TaskItem { state, children });
        }

        Some(Block::TaskList { items })
    }

    pub(crate) fn try_unordered_list(&mut self) -> Option<Block<'a>> {
        if !self.current_line().starts_with("- ") || parse_task_line(self.current_line()).is_some()
        {
            return None;
        }

        let mut items = Vec::new();
        while !self.at_end() {
            let line = self.current_line();
            if !line.starts_with("- ") || parse_task_line(line).is_some() {
                break;
            }

            let rest = &line[2..];
            self.advance_line();
            let mut item_lines = vec![rest];
            item_lines.extend(self.collect_continuation("  "));

            let children = self.parse_sub(&item_lines);
            items.push(ListItem { children });
        }

        Some(Block::UnorderedList { items })
    }

    pub(crate) fn try_ordered_list(&mut self) -> Option<Block<'a>> {
        let (first_num, _) = parse_ordered_line(self.current_line())?;

        let mut items = Vec::new();
        while !self.at_end() {
            let line = self.current_line();
            let Some((_, rest)) = parse_ordered_line(line) else {
                break;
            };

            let indent_str = " ".repeat(line.len() - rest.len());
            self.advance_line();
            let mut item_lines = vec![rest];
            item_lines.extend(self.collect_continuation(&indent_str));

            let children = self.parse_sub(&item_lines);
            items.push(ListItem { children });
        }

        Some(Block::OrderedList {
            start: first_num,
            items,
        })
    }
}

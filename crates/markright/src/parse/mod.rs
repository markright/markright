pub mod block;
pub mod inline;

use crate::ast::block::{Document, FrontMatter};
use std::cell::RefCell;

/// Owns heap strings borrowed by the AST.
#[derive(Default)]
pub struct Bump {
    strings: RefCell<Vec<String>>,
}

impl Bump {
    pub fn new() -> Self {
        Self::default()
    }

    /// Store a string and return a stable borrow.
    pub fn alloc_str(&self, s: String) -> &str {
        let ptr: *const str = &*s;
        self.strings.borrow_mut().push(s);
        // SAFETY: String heap data doesn't move when the String struct is moved into the Vec.
        unsafe { &*ptr }
    }
}

/// Cursor-based parser over a source string.
pub(crate) struct Parser<'a> {
    pub(crate) src: &'a str,
    pub(crate) pos: usize,
    pub(crate) bump: &'a Bump,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str, bump: &'a Bump) -> Self {
        Self { src, pos: 0, bump }
    }

    pub(crate) fn at_end(&self) -> bool {
        self.pos >= self.src.len()
    }

    pub(crate) fn line_at(&self, pos: usize) -> &'a str {
        if pos >= self.src.len() {
            return "";
        }
        let end = self.src[pos..]
            .find('\n')
            .map_or(self.src.len(), |i| pos + i);
        &self.src[pos..end]
    }

    pub(crate) fn current_line(&self) -> &'a str {
        self.line_at(self.pos)
    }

    pub(crate) fn advance_line(&mut self) {
        if let Some(nl) = self.src[self.pos..].find('\n') {
            self.pos += nl + 1;
        } else {
            self.pos = self.src.len();
        }
    }

    pub(crate) fn skip_blank_lines(&mut self) {
        while !self.at_end() && self.current_line().trim().is_empty() {
            self.advance_line();
        }
    }

    pub(crate) fn next_line_start(&self) -> Option<usize> {
        let nl = self.src[self.pos..].find('\n')?;
        let next = self.pos + nl + 1;
        if next < self.src.len() {
            Some(next)
        } else {
            None
        }
    }

    pub(crate) fn consume_fenced_body(
        &mut self,
        is_closing_line: impl Fn(&str) -> bool,
    ) -> &'a str {
        let body_start = self.pos;

        while !self.at_end() {
            if is_closing_line(self.current_line().trim()) {
                break;
            }
            self.advance_line();
        }

        let body = self.src[body_start..self.pos]
            .strip_suffix('\n')
            .unwrap_or(&self.src[body_start..self.pos]);

        if !self.at_end() {
            self.advance_line();
        }

        body
    }

    /// Recursively parse collected lines as blocks.
    pub(crate) fn parse_sub(&self, lines: &[&str]) -> Vec<crate::ast::block::Block<'a>> {
        let joined = lines.join("\n");
        let src = self.bump.alloc_str(joined);
        let mut sub = Parser::new(src, self.bump);
        sub.parse_blocks()
    }
}

/// Parse a document.
pub fn parse<'a>(input: &'a str, bump: &'a Bump) -> Document<'a> {
    let input = if input.contains('\r') {
        bump.alloc_str(input.replace('\r', ""))
    } else {
        input
    };
    let (front_matter, body) = extract_front_matter(input);
    let mut parser = Parser::new(body, bump);
    let children = parser.parse_blocks();
    Document {
        front_matter,
        children,
    }
}

/// Extract front matter delimited by `---\n`. Input must already be CR-stripped.
fn extract_front_matter<'a>(input: &'a str) -> (Option<FrontMatter<'a>>, &'a str) {
    if !input.starts_with("---\n") {
        return (None, input);
    }

    let Some(pos) = input[4..].find("\n---") else {
        return (None, input);
    };

    let raw = &input[4..4 + pos];
    let after_close = 4 + pos + 4;
    let body_start = if input.as_bytes().get(after_close) == Some(&b'\n') {
        after_close + 1
    } else {
        after_close
    };
    (
        Some(FrontMatter { raw }),
        if body_start < input.len() {
            &input[body_start..]
        } else {
            ""
        },
    )
}

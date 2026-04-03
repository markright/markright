pub mod ast;
pub mod extract;
pub mod format;
pub mod lint;
pub mod parse;
pub mod render;

pub use ast::block::Document;
pub use extract::{extract_headings, extract_wikilinks};
pub use format::to_string;
pub use lint::lint;
pub use parse::{Bump, is_markright_syntax, parse};
pub use render::{to_html, to_html_with_options, write_html, write_html_with_options, HtmlOptions};

#[cfg(feature = "schemars")]
pub fn json_schema() -> schemars::Schema {
    schemars::schema_for!(Document<'static>)
}

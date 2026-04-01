pub mod ast;
pub mod format;
pub mod lint;
pub mod parse;
pub mod render;

pub use ast::block::Document;
pub use format::to_string;
pub use lint::lint;
pub use parse::{Bump, parse};
pub use render::{to_html, write_html};

#[cfg(feature = "schemars")]
pub fn json_schema() -> schemars::Schema {
    schemars::schema_for!(Document<'static>)
}

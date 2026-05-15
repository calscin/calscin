//! The main AST declarations of Calscin. AST is used to lower the lexer tokens into parsed structures.

pub mod nodes;
pub mod types;

#[cfg(feature = "parser")]
pub mod parser;

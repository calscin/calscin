//! The main AST declarations of Calscin. AST is used to lower the lexer tokens into parsed structures.

use calsc_utils::alloc::arena::ArenaAllocator;

use crate::nodes::ASTNode;

pub mod ifs;
pub mod imports;
pub mod nodes;
pub mod types;

#[cfg(feature = "parser")]
pub mod parser;

/// The context of the AST, is used to share things around inside of the AST process
pub struct ASTContext {
    pub nodes: ArenaAllocator<ASTNode>,
}

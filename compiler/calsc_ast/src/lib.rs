#![deny(unsafe_code)]

//! The main AST declarations of Calscin. AST is used to lower the lexer tokens into parsed structures.

use calsc_utils::alloc::arena::{ArenaAllocator, ArenaHandle};

use crate::nodes::ASTNode;

pub mod ifs;
pub mod imports;
pub mod nodes;
pub mod path;
pub mod types;

#[cfg(feature = "parser")]
pub mod parser;

pub type ASTArenaAllocator = ArenaAllocator<ASTNode>;

/// The context of the AST, is used to share things around inside of the AST process
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ASTContext {
    pub nodes: ASTArenaAllocator,
    pub tree: Vec<ArenaHandle>,
}

impl ASTContext {
    pub fn new() -> Self {
        Self {
            nodes: ArenaAllocator::new(),
            tree: vec![],
        }
    }
}

impl Default for ASTContext {
    fn default() -> Self {
        ASTContext::new()
    }
}

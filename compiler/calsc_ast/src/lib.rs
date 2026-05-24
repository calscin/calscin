//! The main AST declarations of Calscin. AST is used to lower the lexer tokens into parsed structures.

use std::{cell::RefCell, collections::HashMap};

use calsc_utils::{alloc::arena::ArenaAllocator, hash::HashedString};

use crate::{nodes::ASTNode, refs::ASTArenaReference};

pub mod ifs;
pub mod imports;
pub mod nodes;
pub mod refs;
pub mod types;

#[cfg(feature = "parser")]
pub mod parser;

thread_local! {
    pub static AST_CONTEXT: RefCell<ASTContext> = RefCell::new(ASTContext::new());
}

/// The context of the AST, is used to share things around inside of the AST process
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ASTContext {
    pub nodes: ArenaAllocator<ASTNode, ASTArenaReference>,
    pub tree: HashMap<HashedString, ASTArenaReference>,
    pub tree_order: Vec<HashedString>,
    pub additional_tree: Vec<ASTArenaReference>,
}

impl ASTContext {
    pub fn new() -> Self {
        Self {
            nodes: ArenaAllocator::new(),
            tree: HashMap::new(),
            tree_order: vec![],
            additional_tree: vec![],
        }
    }
}

//! Node reference definitions

use std::{fmt::Debug, ops::Deref};

use calsc_utils::alloc::arena::ArenaAllocatorReference;

use crate::{AST_CONTEXT, nodes::ASTNode};

pub struct ASTArenaReference {
    pub refer: ArenaAllocatorReference,
}

impl Deref for ASTArenaReference {
    type Target = ASTNode;

    fn deref(&self) -> &Self::Target {
        AST_CONTEXT.with_borrow(|f| f.nodes.get_static(self.refer))
    }
}

impl Debug for ASTArenaReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#?}",
            AST_CONTEXT.with_borrow(|f| f.nodes.get_static(self.refer))
        )
    }
}

//! Node reference definitions

#[cfg(feature = "debug")]
use std::fmt::Debug;

use std::ops::Deref;

use calsc_utils::alloc::arena::ArenaAllocatorReference;

use crate::{AST_CONTEXT, nodes::ASTNode};

#[must_use]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct ASTArenaReference {
    pub refer: ArenaAllocatorReference,
}

impl Deref for ASTArenaReference {
    type Target = ASTNode;

    fn deref(&self) -> &Self::Target {
        AST_CONTEXT.with_borrow(|f| f.nodes.get_static(self.clone()))
    }
}

impl From<usize> for ASTArenaReference {
    fn from(value: usize) -> Self {
        ASTArenaReference { refer: value }
    }
}

impl From<ASTArenaReference> for usize {
    fn from(value: ASTArenaReference) -> Self {
        value.refer
    }
}

#[cfg(feature = "debug")]
impl Debug for ASTArenaReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#?}",
            AST_CONTEXT.with_borrow(|f| f.nodes.get_static(self.clone()))
        )
    }
}

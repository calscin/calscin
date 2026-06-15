use std::{fmt::Debug, ops::Deref};

use crate::nodes::ASTNode;

#[derive(Clone, PartialEq)]
pub struct ASTArenaReference {
    reference: &'static ASTNode,
}

impl From<(&'static ASTNode, usize)> for ASTArenaReference {
    fn from(value: (&'static ASTNode, usize)) -> Self {
        ASTArenaReference { reference: value.0 }
    }
}

impl Deref for ASTArenaReference {
    type Target = ASTNode;

    fn deref(&self) -> &Self::Target {
        self.reference
    }
}

impl Debug for ASTArenaReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.reference.fmt(f)
    }
}

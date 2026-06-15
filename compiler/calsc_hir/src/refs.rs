#[cfg(feature = "debug")]
use std::fmt::Debug;

use std::ops::Deref;

use calsc_utils::alloc::arena::ArenaAllocatorReference;

use crate::{HIR_CONTEXT, nodes::HIRNode};

#[must_use]
#[derive(Clone)]
pub struct HIRArenaReference {
    pub refer: &'static HIRNode,
}

impl From<&'static HIRNode> for HIRArenaReference {
    fn from(value: &'static HIRNode) -> Self {
        Self { refer: value }
    }
}

impl Deref for HIRArenaReference {
    type Target = HIRNode;

    fn deref(&self) -> &Self::Target {
        self.refer
    }
}

impl Debug for HIRArenaReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.refer.fmt(f)
    }
}

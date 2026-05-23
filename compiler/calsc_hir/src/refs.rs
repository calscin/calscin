use std::ops::Deref;

use calsc_utils::alloc::arena::ArenaAllocatorReference;

use crate::{HIR_CONTEXT, nodes::HIRNode};

#[must_use]
#[derive(PartialEq, Clone)]
pub struct HIRArenaReference {
    pub refer: ArenaAllocatorReference,
}

impl From<ArenaAllocatorReference> for HIRArenaReference {
    fn from(value: ArenaAllocatorReference) -> Self {
        HIRArenaReference { refer: value }
    }
}

impl From<HIRArenaReference> for ArenaAllocatorReference {
    fn from(value: HIRArenaReference) -> Self {
        value.refer
    }
}

impl Deref for HIRArenaReference {
    type Target = HIRNode;

    fn deref(&self) -> &Self::Target {
        HIR_CONTEXT.with_borrow(|f| f.nodes.get_static(self.clone()))
    }
}

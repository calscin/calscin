#[cfg(feature = "debug")]
use std::fmt::Debug;

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
        HIR_CONTEXT.with(|f| f.borrow().nodes.get_static(self.clone()))
    }
}

#[cfg(feature = "debug")]
impl Debug for HIRArenaReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#?}",
            HIR_CONTEXT.with(|f| f.borrow().nodes.get_static(self.clone()))
        )
    }
}

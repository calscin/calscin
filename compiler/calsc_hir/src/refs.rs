use calsc_utils::alloc::arena::ArenaAllocatorReference;

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

use calsc_modules::treev2::{ModuleTree, entry::TreeEntry};
use calsc_utils::alloc::arena::ArenaAllocator;

pub struct TreeBuildingCtx {
    pub tree: ModuleTree,
    pub arena: ArenaAllocator<TreeEntry>,
}

impl TreeBuildingCtx {
    pub fn new() -> Self {
        Self {
            tree: ModuleTree::new(),
            arena: ArenaAllocator::new(),
        }
    }
}

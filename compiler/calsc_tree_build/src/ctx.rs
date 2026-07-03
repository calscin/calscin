use std::path::PathBuf;

use calsc_modules::{
    path::ModulePath,
    treev2::{ModuleTree, entry::TreeEntry},
};
use calsc_utils::{alloc::arena::ArenaAllocator, hash::HashedString};

pub struct TreeBuildingCtx {
    pub tree: ModuleTree,
    pub arena: ArenaAllocator<TreeEntry>,

    pub current_path: ModulePath,
    pub current_file: PathBuf,
}

impl TreeBuildingCtx {
    pub fn new(package: HashedString) -> Self {
        Self {
            tree: ModuleTree::new(),
            arena: ArenaAllocator::new(),
            current_path: ModulePath::new(package, vec![]),
            current_file: PathBuf::default(),
        }
    }
}

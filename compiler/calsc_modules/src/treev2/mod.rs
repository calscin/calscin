//! The second version of the Calscin module tree.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use calsc_utils::{alloc::arena::ArenaAllocator, hash::HashedString};

use crate::{path::ModulePath, treev2::entry::TreeEntry};

pub mod entry;
pub mod module;

pub struct ModuleTree {
    pub children: HashMap<HashedString, TreeEntry>,
    pub resolved_cache: HashMap<ModulePath, TreeEntry>,
    pub used_files: HashSet<PathBuf>,

    pub entry_arena: ArenaAllocator<TreeEntry>,
}

impl ModuleTree {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            resolved_cache: HashMap::new(),
            used_files: HashSet::new(),

            entry_arena: ArenaAllocator::new(),
        }
    }
}

//! The second version of the Calscin module tree.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use calsc_utils::{alloc::arena::ArenaAllocator, hash::HashedString};

use crate::{
    path::ModulePath,
    treev2::{entry::TreeEntry, module::TreeModule},
};

pub mod entry;
pub mod module;

pub struct ModuleTree {
    pub children: HashMap<HashedString, TreeEntry>,
    pub resolved_cache: HashMap<ModulePath, TreeEntry>,
    pub module_arena: ArenaAllocator<TreeModule>,
    pub used_files: HashSet<PathBuf>,
}

impl ModuleTree {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            resolved_cache: HashMap::new(),
            module_arena: ArenaAllocator::new(),
            used_files: HashSet::new(),
        }
    }
}

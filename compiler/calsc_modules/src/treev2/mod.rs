//! The second version of the Calscin module tree.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_utils::{
    alloc::arena::{ArenaAllocator, ArenaHandle},
    hash::HashedString,
};

use crate::{
    path::ModulePath,
    treev2::{entry::TreeEntry, traverse::TraverseTree},
};

pub mod entry;
pub mod module;
pub mod traverse;

pub struct ModuleTree {
    pub children: HashMap<HashedString, ArenaHandle>,
    pub resolved_cache: HashMap<ModulePath, ArenaHandle>,
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

    pub fn get_entry<'a, S: DiagnosticSource>(
        &'a self,
        path: &ModulePath,
        source: &S,
    ) -> DiagResult<&'a TreeEntry> {
        let mut entry = self.get(path.get_ref(0), path, self, source)?;

        for i in 1..path.get_size() {
            entry = entry.get(path.get_ref(i), path, self, source)?;
        }

        Ok(entry)
    }

    pub fn get_entry_mut<'a, S: DiagnosticSource>(
        &'a mut self,
        path: &ModulePath,
        source: &S,
    ) -> DiagResult<&'a mut TreeEntry> {
        let entry = self.get_entry(path, source)?;
        let handle = self.resolved_cache[&entry.self_path].clone();

        Ok(self.entry_arena.get_mut(&handle))
    }
}

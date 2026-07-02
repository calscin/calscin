//! The second version of the Calscin module tree.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use calsc_diagnostics::{DiagPossible, DiagResult, DiagnosticSource};
use calsc_utils::{
    alloc::arena::{ArenaAllocator, ArenaHandle},
    hash::HashedString,
};

use crate::{
    path::ModulePath,
    treev2::{
        entry::{TreeEntry, TreeEntryKind},
        traverse::TraverseTree,
    },
};

pub mod entry;
pub mod module;
pub mod traverse;

pub struct ModuleTree {
    pub children: HashMap<HashedString, ArenaHandle>,
    pub resolved_cache: HashMap<ModulePath, ArenaHandle>,
    pub used_files: HashSet<PathBuf>,
}

impl ModuleTree {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            resolved_cache: HashMap::new(),
            used_files: HashSet::new(),
        }
    }

    pub fn get_entry<'a, S: DiagnosticSource>(
        &'a self,
        path: &ModulePath,
        arena: &'a ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagResult<&'a TreeEntry> {
        let mut entry = self.get(path.get_ref(0), path, arena, source)?;

        for i in 1..path.get_size() {
            entry = entry.get(path.get_ref(i), path, arena, source)?;
        }

        Ok(entry)
    }

    pub fn get_entry_mut<'a, S: DiagnosticSource>(
        &'a mut self,
        path: &ModulePath,
        arena: &'a mut ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagResult<&'a mut TreeEntry> {
        let entry = self.get_entry(path, arena, source)?;
        let handle = &self.resolved_cache[&entry.self_path];

        Ok(arena.get_mut(handle))
    }

    pub fn append_entry<'a, S: DiagnosticSource>(
        &'a mut self,
        path: &ModulePath,
        val: TreeEntryKind,
        arena: &'a mut ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagPossible {
        if path.get_size() == 1 {
            self.set(path.get(0), path, val, arena, source)?;
        }

        Ok(())
    }
}

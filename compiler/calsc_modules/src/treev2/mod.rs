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
        module::TreeModule,
        traverse::TraverseTree,
    },
};

pub mod entry;
pub mod imports;
pub mod module;
pub mod traverse;

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct ModuleTree {
    pub children: HashMap<HashedString, ArenaHandle>,
    //pub resolved_cache: HashMap<ModulePath, ArenaHandle>, // TODO: ADD BACK LATER
    pub used_files: HashSet<PathBuf>,
}

impl ModuleTree {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            used_files: HashSet::new(),
        }
    }

    pub fn has_entry(&self, path: &ModulePath, arena: &ArenaAllocator<TreeEntry>) -> bool {
        if !self.has(path.get_ref(0)) {
            return false;
        }

        let mut entry = unsafe { self.get_directly(path.get_ref(0), arena) };

        for i in 1..path.get_size() {
            if !entry.has(path.get_ref(i)) {
                return false;
            }

            entry = unsafe { entry.get_directly(path.get_ref(i), arena) };
        }

        true
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

    pub fn get_entry_handle<'a, S: DiagnosticSource>(
        &'a self,
        path: &ModulePath,
        arena: &'a ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagResult<&'a ArenaHandle> {
        let mut entry = self.get_handle(path.get_ref(0), path, source)?;

        for i in 1..path.get_size() {
            entry = arena.get(entry).get_handle(path.get_ref(i), path, source)?;
        }

        Ok(entry)
    }

    pub fn get_entry_mut<'a, S: DiagnosticSource>(
        &'a mut self,
        path: &ModulePath,
        arena: &'a mut ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagResult<&'a mut TreeEntry> {
        let entry_handle = self.get_entry_handle(path, arena, source)?.clone();

        Ok(arena.get_mut(&entry_handle))
    }

    pub fn append_entry<'a, S: DiagnosticSource>(
        &'a mut self,
        path: &ModulePath,
        val: TreeEntryKind,
        arena: &'a mut ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagPossible {
        if path.get_size() == 1 {
            let val = TreeEntry::new(val, path.clone());
            let val = arena.append(val);

            self.set(path.get(0), path, val, source)?;

            return Ok(());
        }

        let mut parent_path = path.clone();
        let last = parent_path.last();

        parent_path.path.pop();

        let val = TreeEntry::new(val, path.clone());
        let val = arena.append(val);

        let parent_ref = self.get_entry_mut(&parent_path, arena, source)?;

        parent_ref.set(last, path, val, source)?;

        Ok(())
    }

    pub fn append_module<'a, S: DiagnosticSource>(
        &'a mut self,
        path: &ModulePath,
        file_path: PathBuf,
        arena: &'a mut ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagPossible {
        self.used_files.insert(file_path.clone());

        let name = path.last();

        println!("Appending module {} at {}", name, path);

        let entry = TreeEntryKind::Module(TreeModule::new(name, file_path));

        self.append_entry(&path, entry, arena, source)
    }
}

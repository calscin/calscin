use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_cannot_find_element_no_closest, build_expected_entry_type},
};
use calsc_utils::{
    alloc::arena::{ArenaAllocator, ArenaHandle},
    hash::HashedString,
};

use crate::{
    path::ModulePath,
    treev2::{
        ModuleTree,
        entry::{TreeEntry, TreeEntryKind},
    },
};

pub trait TraverseTree {
    unsafe fn get_directly<'a>(
        &'a self,
        name: &HashedString,
        arena: &'a ArenaAllocator<TreeEntry>,
    ) -> &'a TreeEntry;

    unsafe fn get_directly_handle<'a>(&'a self, name: &HashedString) -> &'a ArenaHandle;

    fn has(&self, name: &HashedString) -> bool;

    fn set<'a, S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        path: &ModulePath,
        val: ArenaHandle,
        source: &S,
    ) -> DiagPossible;

    fn get<'a, S: DiagnosticSource>(
        &'a self,
        name: &HashedString,
        path: &ModulePath,
        arena: &'a ArenaAllocator<TreeEntry>,
        source: &S,
    ) -> DiagResult<&'a TreeEntry> {
        if !self.has(name) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        unsafe { Ok(self.get_directly(name, arena)) }
    }

    fn get_handle<'a, S: DiagnosticSource>(
        &'a self,
        name: &HashedString,
        path: &ModulePath,
        source: &S,
    ) -> DiagResult<&'a ArenaHandle> {
        if !self.has(name) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        unsafe { Ok(self.get_directly_handle(name)) }
    }
}

impl TraverseTree for TreeEntry {
    fn has(&self, name: &HashedString) -> bool {
        match &self.kind {
            TreeEntryKind::Module(module) => module.children.contains_key(name),
            _ => false,
        }
    }

    fn set<'a, S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        _path: &ModulePath,
        val: ArenaHandle,
        source: &S,
    ) -> DiagPossible {
        match &mut self.kind {
            TreeEntryKind::Module(module) => {
                module.children.insert(name, val);

                Ok(())
            }

            _ => {
                return Err(build_expected_entry_type(
                    &"module".to_string(),
                    &"??".to_string(),
                    source,
                )
                .into());
            }
        }
    }

    unsafe fn get_directly<'a>(
        &'a self,
        name: &HashedString,
        arena: &'a ArenaAllocator<TreeEntry>,
    ) -> &'a TreeEntry {
        match &self.kind {
            TreeEntryKind::Module(module) => arena.get(&module.children[&name]),
            _ => panic!(),
        }
    }

    unsafe fn get_directly_handle<'a>(&'a self, name: &HashedString) -> &'a ArenaHandle {
        match &self.kind {
            TreeEntryKind::Module(module) => &module.children[&name],
            _ => panic!(),
        }
    }
}

impl TraverseTree for ModuleTree {
    fn has(&self, name: &HashedString) -> bool {
        self.children.contains_key(name)
    }

    fn set<'a, S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        _path: &ModulePath,
        val: ArenaHandle,
        _source: &S,
    ) -> DiagPossible {
        self.children.insert(name, val);
        Ok(())
    }

    unsafe fn get_directly<'a>(
        &'a self,
        name: &HashedString,
        arena: &'a ArenaAllocator<TreeEntry>,
    ) -> &'a TreeEntry {
        arena.get(&self.children[&name])
    }

    unsafe fn get_directly_handle<'a>(&'a self, name: &HashedString) -> &'a ArenaHandle {
        &self.children[&name]
    }
}

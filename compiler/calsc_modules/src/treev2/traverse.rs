use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_cannot_find_element_no_closest, build_expected_entry_type},
};
use calsc_utils::hash::HashedString;

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
        tree: &'a ModuleTree,
    ) -> &'a TreeEntry;

    unsafe fn get_directly_mut<'a>(
        &'a mut self,
        name: &HashedString,
        tree: &'a mut ModuleTree,
    ) -> &'a mut TreeEntry;

    fn has(&self, name: &HashedString, tree: &ModuleTree) -> bool;

    fn set<'a, S: DiagnosticSource>(
        &'a mut self,
        name: HashedString,
        path: &ModulePath,
        val: TreeEntryKind,
        tree: &'a mut ModuleTree,
        source: &S,
    ) -> DiagPossible;

    fn get<'a, S: DiagnosticSource>(
        &'a self,
        name: &HashedString,
        path: &ModulePath,
        tree: &'a ModuleTree,
        source: &S,
    ) -> DiagResult<&'a TreeEntry> {
        if !self.has(name, tree) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        unsafe { Ok(self.get_directly(name, tree)) }
    }

    fn get_mut<'a, S: DiagnosticSource>(
        &'a mut self,
        name: &HashedString,
        path: &ModulePath,
        tree: &'a mut ModuleTree,
        source: &S,
    ) -> DiagResult<&'a mut TreeEntry> {
        if !self.has(name, tree) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        unsafe { Ok(self.get_directly_mut(name, tree)) }
    }
}

impl TraverseTree for TreeEntry {
    fn has(&self, name: &HashedString, _tree: &ModuleTree) -> bool {
        match &self.kind {
            TreeEntryKind::Module(module) => module.children.contains_key(name),
            _ => false,
        }
    }

    fn set<'a, S: DiagnosticSource>(
        &'a mut self,
        name: HashedString,
        path: &ModulePath,
        val: TreeEntryKind,
        tree: &'a mut ModuleTree,
        source: &S,
    ) -> DiagPossible {
        match &mut self.kind {
            TreeEntryKind::Module(module) => {
                let val = TreeEntry::new(val, path.clone());
                let val = tree.entry_arena.append(val);

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
        tree: &'a ModuleTree,
    ) -> &'a TreeEntry {
        match &self.kind {
            TreeEntryKind::Module(module) => tree.entry_arena.get(&module.children[&name]),
            _ => panic!(),
        }
    }

    unsafe fn get_directly_mut<'a>(
        &'a mut self,
        name: &HashedString,
        tree: &'a mut ModuleTree,
    ) -> &'a mut TreeEntry {
        match &mut self.kind {
            TreeEntryKind::Module(module) => tree.entry_arena.get_mut(&module.children[&name]),
            _ => panic!(),
        }
    }
}

impl TraverseTree for ModuleTree {
    fn has(&self, name: &HashedString, _tree: &ModuleTree) -> bool {
        self.children.contains_key(name)
    }

    fn set<'a, S: DiagnosticSource>(
        &'a mut self,
        name: HashedString,
        path: &ModulePath,
        val: TreeEntryKind,
        _tree: &'a mut ModuleTree,
        _source: &S,
    ) -> DiagPossible {
        let val = TreeEntry::new(val, path.clone());
        let val = self.entry_arena.append(val);

        self.children.insert(name, val);
        Ok(())
    }

    unsafe fn get_directly<'a>(
        &'a self,
        name: &HashedString,
        _tree: &'a ModuleTree,
    ) -> &'a TreeEntry {
        self.entry_arena.get(&self.children[name])
    }

    unsafe fn get_directly_mut<'a>(
        &'a mut self,
        name: &HashedString,
        _tree: &'a mut ModuleTree,
    ) -> &'a mut TreeEntry {
        self.entry_arena.get_mut(&self.children[name])
    }
}

//! The module tree is the structure used to allow for circular imports safely.
//! The module tree is a structure holding modules in a tree-like structure while holding their inner child.

use std::{collections::HashMap, path::PathBuf};

use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource, PosDiagnosticSource,
    diags::errors::{build_already_in_scope, build_cannot_find_element_no_closest},
};
use calsc_utils::hash::HashedString;

use crate::{
    path::ModulePath,
    tree::{
        entry::{ModuleTreeEntry, TreeModule},
        traversal::ModuleTreeTraversal,
    },
};

pub mod clean;
pub mod collect;
pub mod entry;
pub mod traversal;

/// The module tree
#[derive(Debug)]
pub struct ModuleTree {
    pub entries: HashMap<HashedString, ModuleTreeEntry>,
}

impl ModuleTree {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Traverses to the given destination
    pub fn traverse_to<S: DiagnosticSource>(
        &self,
        path: ModulePath,
        source: &S,
    ) -> DiagResult<ModuleTreeEntry> {
        let mut curr = self.traverse(&path, 0, source)?;

        for i in 1..path.get_size() {
            curr = curr.traverse(&path, i, source)?;
        }

        Ok(curr.clone())
    }

    pub fn traverse_mutably_to<S: DiagnosticSource>(
        &mut self,
        path: ModulePath,
        source: &S,
    ) -> DiagResult<&mut ModuleTreeEntry> {
        let mut curr = self.traverse_mut(&path, 0, source)?;

        for i in 1..path.get_size() {
            curr = curr.traverse_mut(&path, i, source)?;
        }

        Ok(curr)
    }

    pub fn traverse_to_append<S: DiagnosticSource>(
        &mut self,
        path: ModulePath,
        val: ModuleTreeEntry,
        source: &S,
    ) -> DiagPossible {
        if path.get_size() <= 1 {
            self.set(path.last(), val, source)?;
            return Ok(());
        }

        let mut curr = self.traverse_mut(&path, 0, source)?;

        for i in 1..path.get_size() - 1 {
            curr = curr.traverse_mut(&path, i, source)?;
        }

        curr.set(path.last(), val, source)
    }

    pub fn contains(&self, path: ModulePath) -> bool {
        let fake_origin = PosDiagnosticSource::new(Default::default(), Default::default()); // This is fine since normally the tree cannot fail 

        if !self.has(path.get(0)) {
            return false;
        }

        let mut curr = match self.traverse(&path, 0, &fake_origin) {
            Ok(v) => v,
            Err(_) => return false,
        };

        for i in 1..path.get_size() {
            if !curr.has(path.get(i)) {
                return false;
            }

            curr = match curr.traverse(&path, i, &fake_origin) {
                Ok(v) => v,
                Err(_) => return false,
            };
        }

        true
    }
}

impl ModuleTreeTraversal for ModuleTree {
    fn traverse<S: DiagnosticSource>(
        &self,
        path: &ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&ModuleTreeEntry> {
        let val = path.get(ind);

        if !self.entries.contains_key(&val) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        Ok(&self.entries[&val])
    }

    fn traverse_mut<S: DiagnosticSource>(
        &mut self,
        path: &ModulePath,
        ind: usize,
        _source: &S,
    ) -> DiagResult<&mut ModuleTreeEntry> {
        let val = path.get(ind);

        if !self.entries.contains_key(&val) {
            self.entries.insert(
                val.clone(),
                ModuleTreeEntry::Module(TreeModule::new(val.clone())),
            );
        }

        Ok(self.entries.get_mut(&val).unwrap())
    }

    fn set<S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        val: ModuleTreeEntry,
        source: &S,
    ) -> DiagPossible {
        if self.entries.contains_key(&name) {
            return Err(build_already_in_scope(&name, source).into());
        }

        self.entries.insert(name, val);
        Ok(())
    }

    fn has(&self, name: HashedString) -> bool {
        self.entries.contains_key(&name)
    }
}

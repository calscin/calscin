//! The module tree is the structure used to allow for circular imports safely.
//! The module tree is a structure holding modules in a tree-like structure while holding their inner child.

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource, diags::errors::build_cannot_find_element_no_closest,
};
use calsc_utils::hash::HashedString;

use crate::{
    path::ModulePath,
    tree::{entry::ModuleTreeEntry, traversal::ModuleTreeTraversal},
};

pub mod entry;
pub mod traversal;

/// The module tree
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
}

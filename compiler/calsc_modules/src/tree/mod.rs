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
}

impl ModuleTreeTraversal for ModuleTree {
    fn traverse<S: DiagnosticSource>(
        &self,
        path: ModulePath,
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

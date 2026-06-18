//! Definitions related to the traversal of the module tree

use std::path::PathBuf;

use calsc_diagnostics::{DiagPossible, DiagResult, DiagnosticSource};
use calsc_utils::hash::HashedString;

use crate::{path::ModulePath, tree::entry::ModuleTreeEntry};

/// Trait that represents every element that is traversable inside of a module tree
pub trait ModuleTreeTraversal {
    /// Traverses the element inside of the module tree
    fn traverse<S: DiagnosticSource>(
        &self,
        path: &ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&ModuleTreeEntry>;

    /// Traverses the element inside of the module tree or creates it if needed
    fn traverse_mut<S: DiagnosticSource>(
        &mut self,
        path: &ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&mut ModuleTreeEntry>;

    fn set<S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        val: ModuleTreeEntry,
        source: &S,
    ) -> DiagPossible;

    /// Collects the paths contained inside of the  module tree
    fn collect_paths(&self, vec: &mut Vec<PathBuf>);

    fn has(&self, name: HashedString) -> bool;

    fn collect_entries<F>(&self, f: F, entries: &mut Vec<(ModuleTreeEntry, ModulePath)>)
    where
        F: FnOnce(&ModuleTreeEntry) -> bool;
}

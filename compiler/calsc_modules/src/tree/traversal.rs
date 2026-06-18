//! Definitions related to the traversal of the module tree

use calsc_diagnostics::{DiagPossible, DiagResult, DiagnosticSource};
use calsc_utils::hash::HashedString;

use crate::{
    path::ModulePath,
    tree::entry::{ModuleTreeEntry, TreeModule},
};

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

    fn has(&self, name: HashedString) -> bool;

    /// Gets the next module in the path if this module contains a file path.
    fn get_next_module<S: DiagnosticSource>(
        &self,
        path: &ModulePath,
        ind: usize,
        source: &S,
        module: TreeModule,
    ) -> DiagResult<TreeModule>;
}

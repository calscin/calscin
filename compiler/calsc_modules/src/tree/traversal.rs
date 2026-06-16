//! Definitions related to the traversal of the module tree

use calsc_diagnostics::{DiagResult, DiagnosticSource};

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
}

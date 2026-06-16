use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource, diags::errors::build_cannot_find_element_no_closest,
};
use calsc_typing::tree::Type;
use calsc_utils::hash::HashedString;

use crate::{path::ModulePath, tree::traversal::ModuleTreeTraversal};

#[derive(Debug, Clone)]
pub enum ModuleTreeEntry {
    Function(Type, Vec<(HashedString, Type)>),
    Module(TreeModule),
}

/// A module contained inside of the module tree
#[derive(Debug, Clone)]
pub struct TreeModule {
    pub name: HashedString,
    pub children: HashMap<HashedString, ModuleTreeEntry>,
}

impl ModuleTreeEntry {
    /// Checks if the given [`ModuleTreeEntry`] can contain children.
    /// This is used for traversing
    pub fn has_children(&self) -> bool {
        match self {
            Self::Module(_) => true,

            _ => false,
        }
    }
}

impl ModuleTreeTraversal for TreeModule {
    fn traverse<S: DiagnosticSource>(
        &self,
        path: ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&ModuleTreeEntry> {
        let val = path.get(ind);

        if !self.children.contains_key(&val) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        Ok(&self.children[&val])
    }
}

impl ModuleTreeTraversal for ModuleTreeEntry {
    fn traverse<S: DiagnosticSource>(
        &self,
        path: ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&ModuleTreeEntry> {
        match self {
            Self::Module(module) => module.traverse(path, ind, source),

            _ => return Err(build_cannot_find_element_no_closest(&path, source).into()),
        }
    }
}

use std::{collections::HashMap, path::PathBuf};

use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_already_in_scope, build_cannot_find_element_no_closest},
};

use calsc_utils::hash::HashedString;

use crate::{
    lazy::{LazyLoadedType, raw::LazyLoadedRawType},
    path::ModulePath,
    tree::traversal::ModuleTreeTraversal,
};

/// An entry inside of the module tree
#[derive(Debug, Clone)]
pub enum ModuleTreeEntry {
    /// A function that had it's contents loadded. This state is available after and during the filling pass of layer 0.
    FilledFunction(LazyLoadedType, Vec<(HashedString, LazyLoadedType)>),

    /// A type that had it's contents loadded. This state is available after and during the filling pass of layer 0.
    FilledType(LazyLoadedRawType),

    Module(TreeModule),

    /// A function whose content did not load yet.
    EmptyFunction,

    /// A type whose content did not load yet.
    EmptyType,
}

/// A module contained inside of the module tree
#[derive(Debug, Clone)]
pub struct TreeModule {
    pub name: HashedString,
    pub children: HashMap<HashedString, ModuleTreeEntry>,
    pub imported: bool,
    pub path: Option<PathBuf>,
}

impl TreeModule {
    pub fn new(name: HashedString) -> Self {
        Self {
            name,
            children: HashMap::new(),
            imported: false,
            path: None,
        }
    }
}

impl ModuleTreeEntry {
    /// Checks if the given [`ModuleTreeEntry`] can contain children.
    /// This is used for traversing
    pub fn has_children(&self) -> bool {
        matches!(self, Self::Module(_))
    }

    pub fn is_module(&self) -> bool {
        matches!(self, Self::Module(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::FilledType(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Self::FilledFunction(_, _))
    }

    pub fn can_be_replaced(&self) -> bool {
        matches!(self, Self::EmptyType | Self::EmptyFunction)
    }
}

impl ModuleTreeTraversal for TreeModule {
    fn traverse<S: DiagnosticSource>(
        &self,
        path: &ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&ModuleTreeEntry> {
        let val = path.get(ind);

        if !self.children.contains_key(&val) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        Ok(&self.children[&val])
    }

    fn get_next_module<S: DiagnosticSource>(
        &self,
        path: &ModulePath,
        ind: usize,
        source: &S,
        module: TreeModule,
    ) -> DiagResult<TreeModule> {
        let val = path.get(ind);

        let module = if self.path.is_some() {
            self.clone()
        } else {
            module
        };

        if !self.children.contains_key(&val) {
            return Err(build_cannot_find_element_no_closest(&path, source).into());
        }

        self.children[&val].get_next_module(path, ind + 1, source, module)
    }

    fn traverse_mut<S: DiagnosticSource>(
        &mut self,
        path: &ModulePath,
        ind: usize,
        _source: &S,
    ) -> DiagResult<&mut ModuleTreeEntry> {
        let val = path.get(ind);

        if !self.children.contains_key(&val) {
            self.children.insert(
                val.clone(),
                ModuleTreeEntry::Module(TreeModule::new(val.clone())),
            );
        }

        Ok(self.children.get_mut(&val).unwrap())
    }

    fn set<S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        val: ModuleTreeEntry,
        source: &S,
    ) -> DiagPossible {
        if self.children.contains_key(&name) && !self.children[&name].can_be_replaced() {
            return Err(build_already_in_scope(&name, source).into());
        }

        self.children.insert(name, val);
        Ok(())
    }
    fn has(&self, name: HashedString) -> bool {
        self.children.contains_key(&name)
    }
}

impl ModuleTreeTraversal for ModuleTreeEntry {
    fn traverse<S: DiagnosticSource>(
        &self,
        path: &ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&ModuleTreeEntry> {
        match self {
            Self::Module(module) => module.traverse(path, ind, source),

            _ => return Err(build_cannot_find_element_no_closest(&path, source).into()),
        }
    }

    fn get_next_module<S: DiagnosticSource>(
        &self,
        path: &ModulePath,
        ind: usize,
        source: &S,
        module: TreeModule,
    ) -> DiagResult<TreeModule> {
        match self {
            Self::Module(m) => m.get_next_module(path, ind, source, module),
            _ => Ok(module),
        }
    }

    fn traverse_mut<S: DiagnosticSource>(
        &mut self,
        path: &ModulePath,
        ind: usize,
        source: &S,
    ) -> DiagResult<&mut ModuleTreeEntry> {
        match self {
            Self::Module(module) => module.traverse_mut(path, ind, source),

            _ => return Err(build_cannot_find_element_no_closest(&path, source).into()),
        }
    }

    fn set<S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        val: ModuleTreeEntry,
        source: &S,
    ) -> DiagPossible {
        match self {
            Self::Module(module) => module.set(name, val, source),

            _ => return Err(build_already_in_scope(&name, source).into()),
        }
    }

    fn has(&self, name: HashedString) -> bool {
        match self {
            Self::Module(module) => module.has(name),

            _ => false,
        }
    }
}

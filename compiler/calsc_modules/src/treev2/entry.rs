use std::{collections::HashSet, fmt::Display};

use calsc_diagnostics::{DiagResult, DiagnosticSource, diags::errors::build_expected_entry_type};

use crate::{path::ModulePath, treev2::module::TreeModule};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum TreeEntryKind {
    Type,
    PrimitiveType,
    Function,
    Module(TreeModule),
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct TreeEntry {
    pub self_path: ModulePath,

    pub kind: TreeEntryKind,

    pub typing_dependencies: HashSet<ModulePath>,
    pub semantic_dependencies: HashSet<ModulePath>,
}

impl TreeEntry {
    pub fn new(kind: TreeEntryKind, self_path: ModulePath) -> Self {
        Self {
            kind,
            self_path,
            typing_dependencies: HashSet::new(),
            semantic_dependencies: HashSet::new(),
        }
    }

    pub fn get_dependencies(&self) -> HashSet<ModulePath> {
        let mut deps = self.typing_dependencies.clone();

        for dependency in &self.semantic_dependencies {
            deps.insert(dependency.clone());
        }

        deps
    }

    pub fn has_dependency(&self, dep: ModulePath) -> bool {
        self.typing_dependencies.contains(&dep) || self.semantic_dependencies.contains(&dep)
    }

    pub fn has_related_nodes(&self) -> bool {
        matches!(self.kind, TreeEntryKind::Function | TreeEntryKind::Type)
    }
}

impl TreeEntryKind {
    #[inline]
    pub fn is_module(&self) -> bool {
        matches!(self, Self::Module(_))
    }

    pub fn as_module<'a, S: DiagnosticSource>(&'a self, source: &S) -> DiagResult<&'a TreeModule> {
        if !self.is_module() {
            return Err(build_expected_entry_type(&"module".to_string(), self, source).into());
        }

        if let Self::Module(module) = self {
            return Ok(module);
        } else {
            unreachable!()
        }
    }

    pub fn as_module_mut<'a, S: DiagnosticSource>(
        &'a mut self,
        source: &S,
    ) -> DiagResult<&'a mut TreeModule> {
        if !self.is_module() {
            return Err(build_expected_entry_type(&"module".to_string(), self, source).into());
        }

        if let Self::Module(module) = self {
            return Ok(module);
        } else {
            unreachable!()
        }
    }
}

impl Display for TreeEntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Module(module) => write!(f, "module ({})", module.name),
            Self::Function => write!(f, "function"),
            Self::Type => write!(f, "type"),
            Self::PrimitiveType => write!(f, "primitive type"),
        }
    }
}

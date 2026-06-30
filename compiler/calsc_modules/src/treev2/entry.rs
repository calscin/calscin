use std::collections::HashSet;

use crate::{path::ModulePath, treev2::module::TreeModule};

pub enum TreeEntryKind {
    Type,
    Function,
    Module(TreeModule),
}

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
}

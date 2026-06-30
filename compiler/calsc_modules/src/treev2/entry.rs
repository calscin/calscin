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
}

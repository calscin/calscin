use std::{collections::HashMap, path::PathBuf};

use calsc_ast::nodes::ASTNode;
use calsc_diagnostics::{DiagnosticSource, result::CalscinResult};
use calsc_modules::{
    path::ModulePath,
    treev2::{ModuleTree, entry::TreeEntry},
};
use calsc_utils::{alloc::arena::ArenaAllocator, hash::HashedString};

use crate::prelude::apply_prelude;

#[cfg_attr(feature = "debug", derive(Debug))]
pub struct TreeBuildingCtx {
    pub tree: ModuleTree,
    pub arena: ArenaAllocator<TreeEntry>,

    pub related_nodes: HashMap<ModulePath, Vec<ASTNode>>,

    pub current_path: ModulePath,
    pub current_file: PathBuf,
}

impl TreeBuildingCtx {
    pub fn new<S: DiagnosticSource>(package: HashedString, source: &S) -> Self {
        let mut ctx = Self {
            tree: ModuleTree::new(),
            arena: ArenaAllocator::new(),
            related_nodes: HashMap::new(),
            current_path: ModulePath::new(package, vec![]),
            current_file: PathBuf::default(),
        };

        apply_prelude(&mut ctx, source).unwrap_cleanly();

        ctx
    }

    pub fn append_related_node(&mut self, path: ModulePath, node: ASTNode) {
        if self.related_nodes.contains_key(&path) {
            self.related_nodes.get_mut(&path).unwrap().push(node);
        } else {
            self.related_nodes.insert(path, vec![node]);
        }
    }
}

use std::{collections::HashMap, path::PathBuf};

use calsc_ast::{ASTContext, nodes::ASTNode};
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

    pub related_nodes: HashMap<ModulePath, (PathBuf, Vec<ASTNode>)>,
    pub import_nodes: HashMap<ModulePath, Vec<ASTNode>>,
    pub ast_contexts: HashMap<PathBuf, ASTContext>,

    pub is_pkg_enabled: bool,

    /// The base of the module path based on the given file
    pub module_path_base: HashMap<PathBuf, ModulePath>,

    pub current_path: ModulePath,
    pub current_file: PathBuf,
}

impl TreeBuildingCtx {
    pub fn new<S: DiagnosticSource>(
        package: HashedString,
        source: &S,
        is_pkg_enabled: bool,
    ) -> Self {
        let mut ctx = Self {
            tree: ModuleTree::new(),
            arena: ArenaAllocator::new(),
            ast_contexts: HashMap::new(),
            related_nodes: HashMap::new(),
            import_nodes: HashMap::new(),
            module_path_base: HashMap::new(),
            current_path: ModulePath::new(package, vec![]),
            current_file: PathBuf::default(),
            is_pkg_enabled,
        };

        apply_prelude(&mut ctx, source).unwrap_cleanly();

        ctx
    }

    pub fn append_related_node(&mut self, path: ModulePath, file_path: PathBuf, node: ASTNode) {
        if self.related_nodes.contains_key(&path) {
            self.related_nodes.get_mut(&path).unwrap().1.push(node);
        } else {
            self.related_nodes.insert(path, (file_path, vec![node]));
        }
    }

    pub fn append_import_node(&mut self, node: ASTNode) {
        if self.import_nodes.contains_key(&self.current_path) {
            self.import_nodes
                .get_mut(&self.current_path)
                .unwrap()
                .push(node);
        } else {
            self.import_nodes
                .insert(self.current_path.clone(), vec![node]);
        }
    }
}

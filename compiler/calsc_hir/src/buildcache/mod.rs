//! The cache used to store HIR and AST states of built files to avoid recompiling
//! This allows for individual lowering of types and allows for circular imports

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use calsc_ast::nodes::ASTNode;
use calsc_modules::path::ModulePath;
use calsc_typing::types::TypeKind;
use calsc_utils::hash::HashedString;

use crate::buildcache::{entry::BuildCacheEntry, types::ResolvedTypeCache};

pub mod entry;
pub mod types;

pub struct BuildCache {
    pub entries: HashMap<PathBuf, BuildCacheEntry>,

    /// Nodes that are related to the type entry at the given module path.
    /// This includes nodes like:
    /// - Struct declarations
    /// - Struct method decl blocks
    /// - Type aliases when they arrive
    pub nodes_to_entries: HashMap<ModulePath, Vec<ASTNode>>,

    pub type_storage: ResolvedTypeCache,

    /// The used type parameter combinations for
    pub used_type_params: HashMap<ModulePath, HashSet<Vec<TypeKind>>>,
}

impl BuildCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            nodes_to_entries: HashMap::new(),
            type_storage: ResolvedTypeCache::new(),
            used_type_params: HashMap::new(),
        }
    }

    pub fn append_entry(&mut self, path: PathBuf, entry: BuildCacheEntry) {
        self.entries.insert(path, entry);
    }

    pub fn append_used_type_param_combination(
        &mut self,
        path: ModulePath,
        combinations: Vec<TypeKind>,
    ) {
        if !self.used_type_params.contains_key(&path) {
            self.used_type_params.insert(path.clone(), HashSet::new());
        }

        let set_mut = self.used_type_params.get_mut(&path).unwrap();

        set_mut.
    }

    pub fn append_related_node(&mut self, path: ModulePath, node: ASTNode) {
        let mut vec = self.nodes_to_entries.get(&path).unwrap_or(&vec![]).clone();
        vec.push(node);

        self.nodes_to_entries.insert(path, vec);
    }
}

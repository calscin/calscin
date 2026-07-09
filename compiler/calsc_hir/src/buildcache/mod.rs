//! The cache used to store HIR and AST states of built files to avoid recompiling
//! This allows for individual lowering of types and allows for circular imports

use std::collections::{HashMap, HashSet};

use calsc_modules::path::ModulePath;
use calsc_typing::hash::HashedTypeKind;

pub mod types;

pub struct BuildCache {
    pub used_type_params: HashMap<ModulePath, HashSet<Vec<HashedTypeKind>>>,
}

impl BuildCache {
    pub fn new() -> Self {
        Self {
            used_type_params: HashMap::new(),
        }
    }

    pub fn append_used_type_param_combination(
        &mut self,
        path: ModulePath,
        combinations: Vec<HashedTypeKind>,
    ) {
        if !self.used_type_params.contains_key(&path) {
            self.used_type_params.insert(path.clone(), HashSet::new());
        }

        let set_mut = self.used_type_params.get_mut(&path).unwrap();

        set_mut.insert(combinations);
    }
}

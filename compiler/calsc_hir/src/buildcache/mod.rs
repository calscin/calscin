//! The cache used to store HIR and AST states of built files to avoid recompiling
//! This allows for individual lowering of types and allows for circular imports

use std::{collections::HashMap, path::PathBuf};

use crate::buildcache::entry::BuildCacheEntry;

pub mod entry;

pub struct BuildCache {
    pub entries: HashMap<PathBuf, BuildCacheEntry>,
}

impl BuildCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

//! The module tree is the structure used to allow for circular imports safely.
//! The module tree is a structure holding modules in a tree-like structure while holding their inner child.

use std::collections::HashMap;

use calsc_utils::hash::HashedString;

use crate::tree::entry::ModuleTreeEntry;

pub mod entry;

/// The module tree
pub struct ModuleTree {
    pub entries: HashMap<HashedString, ModuleTreeEntry>,
}

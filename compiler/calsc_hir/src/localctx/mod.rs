//! Definitions for the HIR local contexts
//!
//! Local contexts are used in functions to store the following:
//! - Variables (start, end of eras)
//! - Eras / branches
//! - Finshing / ending points
//!
//! Eras are also named branch indexes

use std::collections::{HashMap, HashSet};

use calsc_typing::tree::Type;
use calsc_utils::hash::HashedString;

pub mod vars;

pub struct LocalContext {
    pub name: HashedString,

    /// Main hashmap to convert a string into an actual variable vector index
    pub hash_to_ind: HashMap<HashedString, usize>,

    /// Stores whenever each branch ends. If a branch is contained here, it ended.
    pub branch_ends: HashSet<usize>,

    pub return_type: Option<Type>,

    pub current_branch: usize,
}

impl LocalContext {
    pub fn new(name: HashedString, return_type: Option<Type>) -> Self {
        Self {
            name,
            hash_to_ind: HashMap::new(),
            branch_ends: HashSet::new(),
            return_type,
            current_branch: 0,
        }
    }
}

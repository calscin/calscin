//! Definitions for the HIR local contexts
//!
//! Local contexts are used in functions to store the following:
//! - Variables (start, end of branches)
//! - Branches
//! - Finshing / ending points
//!

use std::collections::{HashMap, HashSet};

use calsc_typing::tree::Type;
use calsc_utils::hash::HashedString;

use crate::localctx::vars::LocalContextVariable;

pub mod vars;

pub struct LocalContext {
    pub name: HashedString,

    /// Main hashmap to convert a string into an actual variable vector index
    pub hash_to_ind: HashMap<HashedString, usize>,
    pub variables: Vec<LocalContextVariable>,

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
            variables: vec![],
            return_type,
            current_branch: 0,
        }
    }

    /// Starts a new branch and returns it's index
    ///
    #[inline(always)]
    pub fn start_branch(&mut self) -> usize {
        self.current_branch += 1;
        self.current_branch
    }

    /// Moves the current branch to the given branch index
    ///
    /// **Warn: Make sure to only use this with branches obtained through [`start_branch`][`LocalContext::start_branch`]**
    ///
    #[inline(always)]
    pub fn move_branch(&mut self, branch: usize) {
        self.current_branch = branch;
    }

    /// Ends the given branch index at the current branch
    ///
    /// **Warn: Make sure to only use this with branches obtained through [`start_branch`][`LocalContext::start_branch`]**
    ///
    #[inline(always)]
    pub fn end_branch(&mut self, branch: usize) {
        self.branch_ends.insert(branch);
    }
}

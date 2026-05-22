//! Definitions for the HIR local contexts
//!
//! Local contexts are used in functions to store the following:
//! - Variables (start, end of branches)
//! - Branches
//! - Finshing / ending points
//!

use std::collections::{HashMap, HashSet};

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{
        build_already_in_scope, build_cannot_find_element, build_cannot_find_element_no_closest,
    },
};
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

    /// Introduces a variable in the given era
    ///
    /// # Errors
    /// Will error if the local context already contains a variable with that name
    ///
    fn introduce_variable_in_branch<K: DiagnosticSource>(
        &mut self,
        key: HashedString,
        t: Type,
        has_default: bool,
        branch: usize,
        origin: &K,
    ) -> DiagResult<usize> {
        if self.hash_to_ind.contains_key(&key) {
            return Err(build_already_in_scope(&*key, origin).into());
        }

        let var = LocalContextVariable::new(t, branch, has_default);

        let ind = self.variables.len();
        self.variables.push(var);

        self.hash_to_ind.insert(key, ind);
        Ok(ind)
    }

    /// Introduces a variable in the current branch
    ///
    /// # Errors
    /// Will error if the local context already contains a variable with that name
    ///
    pub fn introduce_variable<K: DiagnosticSource>(
        &mut self,
        key: HashedString,
        t: Type,
        has_default: bool,
        origin: &K,
    ) -> DiagResult<usize> {
        self.introduce_variable_in_branch(key, t, has_default, self.current_branch, origin)
    }

    /// Introduces a variable in the next branch
    ///
    /// # Errors
    /// Will error if the local context already contains a variable with that name
    ///
    pub fn introduce_variable_next_branch<K: DiagnosticSource>(
        &mut self,
        key: HashedString,
        t: Type,
        has_default: bool,
        origin: &K,
    ) -> DiagResult<usize> {
        self.introduce_variable_in_branch(key, t, has_default, self.current_branch + 1, origin)
    }

    #[inline(always)]
    pub fn is_branch_alive(&self, branch: usize) -> bool {
        return !self.branch_ends.contains(&branch);
    }

    pub fn is_variable_alive(&self, variable_ind: usize) -> bool {
        let start_branch = self.variables[variable_ind].introduced;

        if start_branch > self.current_branch {
            return false; // Handle introduce_variable_next_branch
        }

        self.is_branch_alive(start_branch)
    }

    pub fn obtain<K: DiagnosticSource>(
        &mut self,
        name: HashedString,
        origin: &K,
    ) -> DiagResult<usize> {
        match self.hash_to_ind.get(&name) {
            None => {
                return Err(build_cannot_find_element_no_closest(&*name, origin).into());
            }

            Some(ind) => {
                let ind = *ind;

                if !self.is_variable_alive(ind) {
                    return Err(build_cannot_find_element_no_closest(&*name, origin).into());
                }

                self.variables[ind].introduce_usage();

                Ok(ind)
            }
        }
    }
}

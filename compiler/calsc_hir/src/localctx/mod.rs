//! Definitions for the HIR local contexts
//!
//! Local contexts are used in functions to store the following:
//! - Variables (start, end of branches)
//! - Branches
//! - Finshing / ending points
//!

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{
        build_already_in_scope, build_cannot_find_element_no_closest, build_not_initialized,
        build_variable_unalive,
    },
};
use calsc_typing::tree::Type;
use calsc_utils::{hash::HashedString, pos::FilePosition};

use crate::{globalctx::key::GlobalContextKey, localctx::vars::LocalContextVariable};

pub mod vars;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)] // For MIR
pub struct LocalContext {
    pub name: HashedString,

    pub local_key: GlobalContextKey,

    /// Main hashmap to convert a string into an actual variable vector index
    pub hash_to_ind: HashMap<HashedString, usize>,
    pub variables: Vec<LocalContextVariable>,

    /// Contains whenever ending points have been introduced
    pub ending_points: Vec<usize>,

    /// Branches that cannot use the return trick. Example: if statements without else
    pub contain_unreal_branches: bool,

    /// Stores whenever each branch ends. If a branch is contained here, it ended.
    pub branch_ends: HashMap<usize, usize>,
    pub branch_ends_positions: HashMap<usize, (FilePosition, FilePosition)>,

    pub return_type: Type,

    pub is_main_function: bool,

    pub current_branch: usize,
    pub branch_count: usize,
}

impl LocalContext {
    pub fn new(
        name: HashedString,
        key: GlobalContextKey,
        return_type: Type,
        is_main_function: bool,
    ) -> Self {
        Self {
            name,
            local_key: key,
            hash_to_ind: HashMap::new(),
            branch_ends: HashMap::new(),
            branch_ends_positions: HashMap::new(),
            ending_points: vec![],
            contain_unreal_branches: false,
            variables: vec![],
            return_type,
            current_branch: 0,
            branch_count: 0,
            is_main_function,
        }
    }

    /// Starts a new branch and returns it's index
    ///
    #[inline(always)]
    pub fn start_branch(&mut self) -> usize {
        self.current_branch += 1;
        self.branch_count += 1;

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
    /// **Warn: This doesn't change the current branch so make to change it naturally**
    ///
    #[inline(always)]
    pub fn end_branch<K: DiagnosticSource>(&mut self, branch: usize, origin: &K) {
        self.branch_ends.insert(branch, self.current_branch);
        self.branch_ends_positions
            .insert(branch, (origin.get_start_pos(), origin.get_end_pos()));
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
        mutable: bool,
        has_default: bool,
        branch: usize,
        origin: &K,
    ) -> DiagResult<usize> {
        if self.hash_to_ind.contains_key(&key) {
            return Err(build_already_in_scope(&*key, origin).into());
        }

        let var = LocalContextVariable::new(t, branch, mutable, has_default);
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
        mutable: bool,
        has_default: bool,
        origin: &K,
    ) -> DiagResult<usize> {
        self.introduce_variable_in_branch(key, t, mutable, has_default, self.current_branch, origin)
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
        mutable: bool,
        has_default: bool,
        origin: &K,
    ) -> DiagResult<usize> {
        self.introduce_variable_in_branch(
            key,
            t,
            mutable,
            has_default,
            self.current_branch + 1,
            origin,
        )
    }

    pub fn introduce_variable_assign(&mut self, key: HashedString) {
        self.variables[self.hash_to_ind[&key]]
            .introduced_values
            .insert(self.current_branch);
    }

    /// Checks if the given branch is currently alive or not.
    #[inline(always)]
    pub fn is_branch_alive(&self, branch: usize) -> bool {
        return !self.branch_ends.contains_key(&branch);
    }

    /// Checks if the variable corresponding to the given index is still currently alive.
    pub fn is_variable_alive(&self, variable_ind: usize) -> bool {
        let start_branch = self.variables[variable_ind].introduced;

        if start_branch > self.current_branch {
            return false; // Handle introduce_variable_next_branch
        }

        self.is_branch_alive(start_branch)
    }

    /// Obtains a variable index from the variable name.
    ///
    /// # Errors
    /// This function will error if the variable wasn't found or isn't alive anymore.
    ///
    pub fn obtain<K: DiagnosticSource>(&self, name: HashedString, origin: &K) -> DiagResult<usize> {
        match self.hash_to_ind.get(&name) {
            None => {
                return Err(build_cannot_find_element_no_closest(&*name, origin).into());
            }

            // TODO: add usage tracking
            Some(ind) => {
                let ind = *ind;

                if !self.is_variable_alive(ind) {
                    let introdued = self.variables[ind].introduced;
                    let branch_end = self.branch_ends[&introdued];
                    let positions = self.branch_ends_positions[&branch_end].clone();
                    let start = positions.0;
                    let end = positions.1;

                    return Err(build_variable_unalive(
                        &name,
                        &introdued,
                        &branch_end,
                        origin,
                        start,
                        end,
                    )
                    .into());
                }

                if !self.has_variable_value(ind) {
                    return Err(build_not_initialized(&*name, origin).into());
                }

                //self.variables[ind].introduce_usage();

                Ok(ind)
            }
        }
    }

    pub fn has_variable_value(&self, ind: usize) -> bool {
        let var = &self.variables[ind];

        if var.has_default {
            return true;
        }

        for branch in var.introduced_values.iter() {
            if self.is_branch_alive(*branch) {
                return true;
            }
        }

        return false;
    }

    #[inline(always)]
    pub fn introduce_ending_point(&mut self) {
        self.ending_points.push(self.current_branch);
    }

    pub fn is_code_in_branch_alive(&self, branch: usize) -> bool {
        for ending in &self.ending_points {
            let end = match self.branch_ends.get(&ending) {
                Some(v) => *v,
                None => *ending,
            };

            if branch >= end {
                return false;
            }
        }

        true
    }

    /// Determines if the function meets the ending points requirements.
    ///
    /// The requirement is that every simple branch should have an ending point active inside of it.
    ///
    /// This function first checks if the current branch have an active return point, if it does. It is valid.
    /// If the current branch isn't qualified as "unreal", if then checks every branch before to check if they have an active return point. If all of them do, then it is valid
    ///
    ///
    pub fn meets_ending_point_requirement(&self) -> bool {
        if self.return_type == Type::Void {
            return true;
        }

        if !self.is_code_in_branch_alive(self.current_branch) {
            return true;
        }

        // If every branch before the current branch are stopped, then the code is okay as well

        if !self.contain_unreal_branches && self.branch_count >= 1 {
            for i in 0..self.current_branch {
                if self.is_code_in_branch_alive(i) {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }
}

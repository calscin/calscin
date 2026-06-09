//! Definitions for the HIR global context
//! The global context holds everything static such as:
//! - Types ([`BaseTypeInstance`][`calsc_typing::base::instance::BaseTypeInstance`])
//! - Functions (signatures, local contexts, declaration references)
//! - Struct functions (signatures, local contexts, declaration references)

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{
        build_already_in_scope, build_cannot_find_element, build_cannot_find_element_no_closest,
    },
};

use calsc_utils::str::levenshtein;

use crate::globalctx::{key::GlobalContextKey, vals::GlobalContextValue};

pub mod key;
pub mod vals;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)] // For MIR
pub struct GlobalContext {
    pub key_to_ind: HashMap<GlobalContextKey, usize>,
    pub values: Vec<GlobalContextValue>,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            key_to_ind: HashMap::new(),
            values: vec![],
        }
    }

    /// Appends the given value inside of the [`GlobalContext`] with the given key
    ///
    /// # Errors
    /// This function will error at the given origin if there is already an entry related to the given key.
    ///
    pub fn append<K: DiagnosticSource>(
        &mut self,
        key: GlobalContextKey,
        value: GlobalContextValue,
        origin: &K,
    ) -> DiagResult<usize> {
        if self.key_to_ind.contains_key(&key) {
            return Err(build_already_in_scope(&key, origin).into());
        }

        let ind = self.values.len();

        self.key_to_ind.insert(key, ind);
        self.values.push(value);

        Ok(ind)
    }

    /// Gets the entry at the given key as a [`GlobalContextValue`] reference
    ///
    /// # Error
    /// This function will error at the given origin if there is no entry related to the given key.
    ///
    pub fn get_entry<K: DiagnosticSource>(
        &self,
        key: GlobalContextKey,
        origin: &K,
    ) -> DiagResult<&GlobalContextValue> {
        if !self.key_to_ind.contains_key(&key) {
            let closest = get_closest_key(self, key.clone());

            if closest.is_some() {
                unsafe {
                    return Err(build_cannot_find_element(
                        &*key.name,
                        &*closest.unwrap_unchecked().name,
                        origin,
                    )
                    .into());
                }
            } else {
                return Err(build_cannot_find_element_no_closest(&*key.name, origin).into());
            }
        }

        Ok(&self.values[self.key_to_ind[&key]])
    }

    /// Mutates the given entry at the given key according to the given mutation function.
    ///
    /// # Error
    /// This function will error at the given origin if there is no entry related to the given key.
    ///
    pub fn mutate_entry<K: DiagnosticSource, F, R>(
        &mut self,
        key: GlobalContextKey,
        func: F,
        origin: &K,
    ) -> DiagResult<R>
    where
        F: FnOnce(&mut GlobalContextValue) -> R,
    {
        if !self.key_to_ind.contains_key(&key) {
            return Err(build_cannot_find_element_no_closest(&*key.name, origin).into());
        }

        let entry = &mut self.values[self.key_to_ind[&key]];

        Ok(func(entry))
    }
}

/// Gets the closest key in the [`GlobalContext`] from the given key using the Levenshtein algorithm
fn get_closest_key(ctx: &GlobalContext, key: GlobalContextKey) -> Option<GlobalContextKey> {
    let mut closest_score: usize = usize::MAX;
    let mut closest: Option<GlobalContextKey> = None;

    for k in ctx.key_to_ind.keys() {
        let score = levenshtein(&*key.name, &*k.name);

        if closest_score > score {
            closest_score = score;
            closest = Some(k.clone());
        }
    }

    closest
}

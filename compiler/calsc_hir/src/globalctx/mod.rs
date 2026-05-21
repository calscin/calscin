//! Definitions for the HIR global context
//! The global context holds everything static such as:
//! - Types ([`BaseTypeInstance`][`calsc_typing::base::instance::BaseTypeInstance`])
//! - Functions (signatures, local contexts, declaration references)

use std::collections::HashMap;

use calsc_diagnostics::{DiagResult, DiagnosticSource, diags::errors::build_already_in_scope};

use crate::globalctx::{key::GlobalContextKey, vals::GlobalContextValue};

pub mod key;
pub mod vals;

pub struct GlobalContext {
    key_to_ind: HashMap<GlobalContextKey, usize>,
    pub values: Vec<GlobalContextValue>,
}

impl GlobalContext {
    pub fn new() -> Self {
        Self {
            key_to_ind: HashMap::new(),
            values: vec![],
        }
    }

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
}

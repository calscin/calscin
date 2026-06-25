//! Declarations for type parameters

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_already_in_scope, build_cannot_find_element_no_closest},
};
use calsc_utils::hash::HashedString;

/// This is a safe handle from a type parameter stored inside of a [`TypeParamCtx`] this enforces that type parameters go trough the expected path.
pub struct TypeParameterId(usize);

pub struct TypeParamCtx {
    params: HashMap<HashedString, HeldTypeParam>,
}

struct HeldTypeParam {
    name: HashedString,
    id: usize,
}

impl TypeParamCtx {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Gets the type parameter from the name
    ///
    /// # Errors
    /// This function will error if the type parameter doesn't exist, to avoid this, use [`TypeParamCtx::has_type_parameter`]
    ///
    pub fn get_type_param<S: DiagnosticSource>(
        &self,
        name: &HashedString,
        source: &S,
    ) -> DiagResult<TypeParameterId> {
        if !self.params.contains_key(name) {
            return Err(build_cannot_find_element_no_closest(&name, source).into());
        }

        Ok(TypeParameterId(self.params[name].id))
    }

    /// Appends a type parameter inside of the context
    ///
    /// # Errors
    /// This function will error if a type parameter with the given name already exists.
    ///
    pub fn append_type_param<S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        source: &S,
    ) -> DiagResult<TypeParameterId> {
        if self.params.contains_key(&name) {
            return Err(build_already_in_scope(&name, source).into());
        }

        let id = self.params.len();

        self.params.insert(name.clone(), HeldTypeParam { name, id });

        Ok(TypeParameterId(id))
    }

    pub fn has_type_parameter(&self, name: &HashedString) -> bool {
        self.params.contains_key(name)
    }
}

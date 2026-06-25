//! Declarations for type parameters

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_already_in_scope, build_cannot_find_element_no_closest},
};
use calsc_utils::hash::HashedString;

/// This is a safe handle from a type parameter stored inside of a [`TypeParamCtx`] this enforces that type parameters go trough the expected path.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
pub struct TypeParameterId(usize, pub HashedString);

pub struct TypeParamCtx {
    params: Vec<HeldTypeParam>,
    current_params: HashMap<HashedString, usize>,
    param_group: usize,
}

struct HeldTypeParam {
    name: HashedString,
    id: usize,
    group: usize,
}

impl TypeParamCtx {
    pub fn new() -> Self {
        Self {
            params: vec![],
            current_params: HashMap::new(),
            param_group: 0,
        }
    }

    pub fn start_param_group(&mut self) -> usize {
        self.param_group += 1;
        self.param_group
    }

    pub fn end_group(&mut self) {
        self.current_params.clear();
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
        if !self.current_params.contains_key(name) {
            return Err(build_cannot_find_element_no_closest(&name, source).into());
        }

        Ok(TypeParameterId(self.current_params[name], name.clone()))
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
        if self.current_params.contains_key(&name) {
            return Err(build_already_in_scope(&name, source).into());
        }

        let id = self.params.len();

        let held_param = HeldTypeParam {
            name: name.clone(),
            id,
            group: self.param_group,
        };

        self.current_params.insert(name.clone(), id);

        self.params.push(held_param);

        Ok(TypeParameterId(id, name))
    }

    pub fn has_type_parameter(&self, name: &HashedString) -> bool {
        self.current_params.contains_key(name)
    }
}

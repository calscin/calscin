//! Definition for structures in the typing system

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagPossible, DiagnosticSource, diags::errors::build_type_already_has_field,
};
use calsc_utils::hash::HashedString;

use crate::types::TypeKind;

/// Represents a named field.
pub struct NamedField(pub HashedString, pub TypeKind);

/// Represents an unnamed field.
pub struct UnNamedField(pub TypeKind);

/// A container to hold fields. This container handles both [`NamedField`] and [`UnNamedField`] fields.
pub struct FieldContainer {
    pub(crate) fields: HashMap<HashedString, UnNamedField>,
}

impl FieldContainer {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn append_named<S: DiagnosticSource>(
        &mut self,
        named: NamedField,
        source: &S,
    ) -> DiagPossible {
        if self.fields.contains_key(&named.0) {
            return Err(build_type_already_has_field(&named.0, source).into());
        }

        self.fields.insert(named.0.clone(), named.into());

        Ok(())
    }

    pub fn append_unnamed<S: DiagnosticSource>(
        &mut self,
        unnamed: UnNamedField,
        source: &S,
    ) -> DiagPossible {
        let named = NamedField(format!("{}", self.fields.len()).into(), unnamed.0);

        self.append_named(named, source)
    }
}

impl Into<UnNamedField> for NamedField {
    fn into(self) -> UnNamedField {
        UnNamedField(self.1)
    }
}

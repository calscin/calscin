//! Definition for structures in the typing system

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagPossible, DiagnosticSource, diags::errors::build_type_already_has_field,
};
use calsc_modules::path::ModulePath;
use calsc_utils::hash::HashedString;

use crate::{ctx::TypeCtx, traits::FieldedType, types::TypeKind};

/// Represents a named field.
pub struct NamedField(pub HashedString, pub TypeKind);

/// Represents an unnamed field.
pub struct UnNamedField(pub TypeKind);

/// A container to hold fields. This container handles both [`NamedField`] and [`UnNamedField`] fields.
pub struct FieldContainer {
    pub(crate) fields: HashMap<HashedString, UnNamedField>,
}

/// A container that holds information about a struct
pub struct StructContainer {
    pub name: HashedString,
    pub module: ModulePath,

    pub fields: FieldContainer,
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

impl FieldedType for FieldContainer {
    fn has_field(&self, name: &HashedString, _ctx: &TypeCtx) -> bool {
        self.fields.contains_key(&name)
    }

    unsafe fn get_field(&self, field: &HashedString, _ctx: &TypeCtx) -> TypeKind {
        self.fields[&field].0.clone()
    }
}

impl Into<UnNamedField> for NamedField {
    fn into(self) -> UnNamedField {
        UnNamedField(self.1)
    }
}

impl PartialEq for StructContainer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.module == other.module
    }
}

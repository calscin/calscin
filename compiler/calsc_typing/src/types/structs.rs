//! Definition for structures in the typing system

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagPossible, DiagnosticSource, diags::errors::build_type_already_has_field,
};
use calsc_modules::path::ModulePath;
use calsc_utils::hash::HashedString;

use crate::{ctx::TypeCtx, traits::FieldedType, types::TypeKind};

/// Represents a named field.
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct NamedField(pub HashedString, pub TypeKind);

/// Represents an unnamed field.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct UnNamedField(pub TypeKind);

/// A container to hold fields. This container handles both [`NamedField`] and [`UnNamedField`] fields.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct FieldContainer {
    pub(crate) fields: HashMap<HashedString, (UnNamedField, usize)>,
    pub(crate) fields_order: Vec<HashedString>,
}

/// A container that holds information about a struct
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct StructContainer {
    pub name: HashedString,
    pub module: ModulePath,

    pub fields: FieldContainer,
}

impl StructContainer {
    pub fn new(name: HashedString, path: ModulePath) -> Self {
        Self {
            name,
            module: path,
            fields: FieldContainer::new(),
        }
    }
}

impl FieldContainer {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            fields_order: vec![],
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

        self.fields_order.push(named.0.clone());
        self.fields
            .insert(named.0.clone(), (named.into(), self.fields_order.len() - 1));

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

    fn get_fields(&self, _ctx: &TypeCtx) -> Vec<HashedString> {
        self.fields_order.clone()
    }

    unsafe fn get_field(&self, field: &HashedString, _ctx: &TypeCtx) -> TypeKind {
        self.fields[&field].0.0.clone()
    }

    fn get_field_index(&self, field: &HashedString, _ctx: &TypeCtx) -> usize {
        self.fields[&field].1
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

//! Structured type definitions

use std::{collections::HashMap, hash::Hash};

use calsc_diagnostics::diags::errors::build_already_in_scope;
use calsc_utils::hash::HashedString;

use crate::{FieldHavingType, MutableFieldHavingType, tree::Type};

/// Represents a `struct` type container.
/// Holds information such as the name of the type and fields
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct BaseStructContainer {
    pub name: HashedString,

    pub fields: HashMap<HashedString, Type>,
    pub field_order: Vec<HashedString>,
}

impl BaseStructContainer {
    /// Creates a new [`BaseStructContainer`] based on the given name
    pub fn new(name: HashedString) -> Self {
        Self {
            name,
            fields: HashMap::new(),
            field_order: vec![],
        }
    }
}

impl Hash for BaseStructContainer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for BaseStructContainer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl FieldHavingType for BaseStructContainer {
    fn has_field(&self, name: HashedString) -> bool {
        self.fields.contains_key(&name)
    }

    fn get_fields(&self) -> Vec<HashedString> {
        self.field_order.clone()
    }

    fn get_field_type(&self, name: HashedString) -> Type {
        self.fields[&name].clone()
    }
}

impl MutableFieldHavingType for BaseStructContainer {
    fn add_field<K: calsc_diagnostics::DiagnosticSource>(
        &mut self,
        name: HashedString,
        ty: Type,
        source: &K,
    ) -> calsc_diagnostics::DiagPossible {
        if self.fields.contains_key(&name) {
            return Err(build_already_in_scope(&*name, source).into());
        }

        self.field_order.push(name.clone());
        self.fields.insert(name, ty);
        Ok(())
    }
}

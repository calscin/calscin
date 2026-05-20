//! Declarations for the type tree

use calsc_utils::hash::HashedString;

use crate::{FieldHavingType, base::instance::BaseTypeInstance};

/// The actual type used for typing in Calscin. Allows for nested references and arrays with base types
#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    /// Represents a basic type
    Base(BaseTypeInstance),

    /// Represents a type parameter
    TypeParameter {
        name: HashedString,
        param_ind: usize,
    },

    /// Represents a reference. By default every reference is mutable. This will be changed in future releases
    Reference { mutable: bool, inner: Box<Type> },

    /// Represents an array of a given size
    Array { size: usize, inner: Box<Type> },
}

impl FieldHavingType for Type {
    fn get_field_type(&self, name: HashedString) -> Type {
        match self {
            Self::Array { .. } => panic!("Cannot find field"),
            Self::TypeParameter { .. } => panic!("Cannot find field"),
            Self::Reference { mutable: _, inner } => inner.get_field_type(name),
            Self::Base(instance) => instance.get_field_type(name),
        }
    }

    fn has_field(&self, name: HashedString) -> bool {
        match self {
            Self::Array { .. } => false,
            Self::TypeParameter { .. } => false,
            Self::Reference { mutable: _, inner } => inner.has_field(name),
            Self::Base(instance) => instance.has_field(name),
        }
    }
}

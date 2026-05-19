//! Definitions for the kind of base type. A base type can also hold information such as functions and more

use calsc_utils::hash::HashedString;

use crate::{FieldHavingType, base::structs::BaseStructContainer, tree::Type};

#[derive(PartialEq, Clone, Debug)] // Remove this and replace it with a custom implementation whenever structs are added
pub enum BaseTypeKind {
    /// An integer type that is possibly signed
    Integer {
        signed: bool,
    },

    /// A floating type that is possibly signed
    Floating {
        signed: bool,
    },

    Struct(BaseStructContainer),

    /// A boolean type
    Boolean,
}

impl BaseTypeKind {
    /// Get the amount of required size parameters to create a base type.
    /// Is used by [`BaseType::new`][`crate::base::BaseType::new`]
    pub fn get_required_size_parameters(&self) -> usize {
        match self {
            Self::Integer { .. } | Self::Floating { .. } => 1,
            _ => 0,
        }
    }
}

impl FieldHavingType for BaseTypeKind {
    fn has_field(&self, name: HashedString) -> bool {
        match self {
            Self::Struct(container) => container.has_field(name),
            _ => false,
        }
    }

    fn get_field_type(&self, name: HashedString) -> Type {
        match self {
            Self::Struct(container) => container.get_field_type(name),
            _ => panic!("Field {} not found on type", *name),
        }
    }
}

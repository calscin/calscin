//! Definitions for the kind of base type. A base type can also hold information such as functions and more

use calsc_utils::hash::HashedString;

use crate::{
    FieldHavingType, MutableFieldHavingType, base::structs::BaseStructContainer, tree::Type,
};

#[derive(PartialEq, Clone, Hash)] // Remove this and replace it with a custom implementation whenever structs are added
#[cfg_attr(feature = "debug", derive(Debug))]
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

    /// A string type
    String,

    /// A char type
    Char,

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

    pub fn get_name(&self) -> HashedString {
        let s = match self {
            Self::Boolean => "bool",
            Self::Char => "char",
            Self::String => "str",
            Self::Floating { signed } => {
                if *signed {
                    "f"
                } else {
                    "uf"
                }
            }

            Self::Integer { signed } => {
                if *signed {
                    "s"
                } else {
                    "u"
                }
            }

            Self::Struct(container) => &container.name,
        };

        s.into()
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

impl MutableFieldHavingType for BaseTypeKind {
    fn add_field<K: calsc_diagnostics::DiagnosticSource>(
        &mut self,
        name: HashedString,
        ty: Type,
        source: &K,
    ) -> calsc_diagnostics::DiagPossible {
        match self {
            Self::Struct(container) => container.add_field(name, ty, source),

            _ => panic!("Fields cannot be added onto this type"),
        }
    }
}

//! Definitions for the kind of base type. A base type can also hold information such as functions and more

use calsc_utils::hash::HashedString;

use crate::{
    FieldHavingType, MutableFieldHavingType, TransmutableType, base::structs::BaseStructContainer,
    tree::Type,
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

    pub fn is_int(&self) -> bool {
        match self {
            Self::Integer { .. } => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::Floating { .. } => true,
            _ => false,
        }
    }

    pub fn is_numerical_lit(&self) -> bool {
        match self {
            Self::Integer { .. } | Self::Floating { .. } => true,
            _ => false,
        }
    }

    pub fn get_signed_state(&self) -> bool {
        match self {
            Self::Integer { signed } => *signed,
            Self::Floating { signed } => *signed,
            _ => panic!(),
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

    fn get_field_index(&self, name: HashedString) -> usize {
        match self {
            Self::Struct(container) => container.get_field_index(name),
            _ => panic!(),
        }
    }

    fn get_fields(&self) -> Vec<HashedString> {
        match self {
            Self::Struct(container) => container.get_fields(),
            _ => vec![],
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

impl TransmutableType for BaseTypeKind {
    fn can_transmute(&self, into: Self) -> bool {
        if self == &into {
            return true;
        }

        match (self, into) {
            (BaseTypeKind::Integer { signed }, BaseTypeKind::Integer { .. }) => !*signed, // Allow unsigned -> signed convertion
            (BaseTypeKind::Floating { signed }, BaseTypeKind::Floating { .. }) => !*signed, // Allow unsigned -> signed convertion,

            _ => false,
        }
    }

    fn can_cast(&self, into: Self) -> bool {
        if self == &into {
            return true;
        }

        match (self, into) {
            (
                BaseTypeKind::Integer { signed },
                BaseTypeKind::Floating {
                    signed: into_signed,
                },
            ) => *signed == into_signed,

            (
                BaseTypeKind::Floating { signed },
                BaseTypeKind::Integer {
                    signed: into_signed,
                },
            ) => *signed == into_signed,

            (BaseTypeKind::Integer { .. }, BaseTypeKind::Integer { .. }) => true,
            (BaseTypeKind::Floating { .. }, BaseTypeKind::Floating { .. }) => true,

            _ => false,
        }
    }

    fn can_transmute_weakly(&self, into: Self) -> bool {
        self.can_transmute(into)
    }
}

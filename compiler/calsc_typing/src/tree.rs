//! Declarations for the type tree

use calsc_utils::hash::HashedString;

use crate::{
    FieldHavingType, TransmutableType,
    base::instance::BaseTypeInstance,
    func::{DeclBlockAffectedType, TypeSignature},
};

/// The actual type used for typing in Calscin. Allows for nested references and arrays with base types
#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
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

impl Type {
    /// Gets the inner type contained in the given type
    ///
    /// # Panics
    /// This function will panic if the type doesn't hold an inner type
    ///
    pub fn get_inner(&self) -> Type {
        match self {
            Self::Reference { mutable: _, inner } => *inner.clone(),
            Self::Array { size: _, inner } => *inner.clone(),

            _ => panic!("The type {} doesn't hold any inner type", self),
        }
    }

    /// Checks if the type is real or not.
    ///
    /// A type is real as long as long as it represents something concrete
    pub fn is_real(&self) -> bool {
        match self {
            Self::Array { size: _, inner } => inner.is_real(),
            Self::Reference { mutable: _, inner } => inner.is_real(),
            Self::TypeParameter { .. } => false,
            Self::Base(_) => true,
        }
    }
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

impl DeclBlockAffectedType for Type {
    fn has_function(&self, name: HashedString) -> bool {
        match self {
            Self::Array { .. } => false,
            Self::TypeParameter { .. } => false,
            Self::Reference { mutable: _, inner } => inner.has_function(name),
            Self::Base(instance) => instance.has_field(name),
        }
    }

    fn get_func_signature(&self, name: HashedString) -> TypeSignature {
        match self {
            Self::Array { .. } => panic!("Cannot find function!"),
            Self::TypeParameter { .. } => panic!("Cannot find function!"),
            Self::Reference { mutable: _, inner } => inner.get_func_signature(name),
            Self::Base(instance) => instance.get_func_signature(name),
        }
    }
}

impl TransmutableType for Type {
    fn can_transmute(&self, into: Self) -> bool {
        if !self.is_real() || into.is_real() {
            return false;
        }

        match (self, into) {
            (
                Self::Array { size, inner },
                Self::Array {
                    size: size2,
                    inner: inner2,
                },
            ) => *size == size2 && inner.can_transmute(*inner2),

            (
                Self::Reference { mutable, inner },
                Self::Reference {
                    mutable: into_mutable,
                    inner: inner2,
                },
            ) => *mutable == into_mutable && inner.can_transmute(*inner2),

            (Self::Base(base), Self::Base(into_base)) => base.can_transmute(into_base),

            _ => false,
        }
    }
}

//! Declarations for the type tree

use calsc_utils::hash::HashedString;

use crate::base::instance::BaseTypeInstance;

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

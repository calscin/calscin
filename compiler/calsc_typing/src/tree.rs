//! Declarations for the type tree

use crate::base::BaseType;

/// The actual type used for typing in Calscin. Allows for nested references and arrays with base types
#[derive(PartialEq)]
pub enum Type {
    /// Represents a basic type
    Base(BaseType),

    /// Represents a reference. By default every reference is mutable. This will be changed in future releases
    Reference { mutable: bool, inner: Box<Type> },

    /// Represents an array of a given size
    Array { size: usize, inner: Box<Type> },
}

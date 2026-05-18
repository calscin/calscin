//! Declaration for functions

use calsc_utils::hash::HashedString;

use crate::tree::Type;

/// Represents a function inside of the typing system
pub struct TypedFunction {
    pub name: HashedString,

    pub arguments: Vec<Type>,
    pub return_type: Option<Type>,
}

impl TypedFunction {
    /// Creates a new [`TypedFunction`] based on the given name, arguments and return type
    pub fn new(name: String, arguments: Vec<Type>, return_type: Option<Type>) -> Self {
        Self {
            name: name.into(),
            arguments,
            return_type,
        }
    }
}

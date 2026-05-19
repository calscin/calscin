//! Definitions for type parameters
//!

use crate::tree::Type;
use calsc_utils::hash::HashedString;

/// Represents a type that can have type parameters
pub trait TypeParameterHaving {
    /// Checks if the type has a type parameter of the given name
    fn has_type_parameter(&self, name: HashedString) -> bool;

    /// Gets the type parameter's type in order to use it based on the given name.
    ///
    /// # Panics
    /// This function will panic if the type parameter doesn't exist in the type.
    /// Make sure to use [`has_type_parameter`][`TypeParameterHaving::has_type_parameter`] first.
    ///
    fn get_type_parameter_type(&self, name: HashedString) -> Type;
}

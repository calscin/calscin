use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_utils::hash::HashedString;

use crate::tree::Type;

pub mod base;
pub mod func;
pub mod tree;

/// Represents a type that has fields.
pub trait FieldHavingType {
    /// Checks if the type has a field with the given name
    fn has_field(&self, name: HashedString) -> bool;

    /// Gets the field with the given name's type.
    ///
    /// # Panic
    /// This function will panic if the field doesn't exist.
    /// Make sure to use [`has_field`][`FieldHavingType::has_field`] before using this function
    ///
    fn get_field_type(&self, name: HashedString) -> Type;
}

/// Same as [`FieldHavingType`] but contains mutable functions
pub trait MutableFieldHavingType {
    /// Adds a field to the type with the given name and type
    ///
    /// # Panics
    ///	This function will panic if the operation is not supported on the type
    ///
    /// # Errors
    /// This function will return an error if the fiels is already present in the time
    fn add_field<K: DiagnosticSource>(
        &mut self,
        name: HashedString,
        ty: Type,
        source: &K,
    ) -> DiagPossible;
}

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

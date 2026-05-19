use calsc_diagnostics::DiagPossible;
use calsc_utils::hash::HashedString;

use crate::tree::Type;

pub mod base;
pub mod func;
pub mod tree;

/// Represents a type that has fields.
pub trait FieldHavingType {
    /// Adds a field to the type with the given name and type
    ///
    /// # Errors
    /// This function will return an error if the fiels is already present in the time
    fn add_field(&mut self, name: HashedString, ty: Type) -> DiagPossible;

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

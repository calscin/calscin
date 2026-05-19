//! Definitions for type parameters
//!

use crate::{base::instance::BaseTypeInstance, tree::Type};
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

/// Resolves a [`Type`] that is potentially a type parameter into a clean [`Type`] based on the given instance's type parameters.
///
/// This function handles type parameters and lowers the type into a type parameter-less version. This function guarantees the output to be type parameter less
///
/// # Panics
/// This function may panic if the type parameter type has an parameter index outside of the range of parameter type of the given instance.
/// To avoid this, make sure to provide the [`BaseTypeInstance`] of the given type instead of a random one.
///
/// # Returns
/// Returns the lowered type as a [`Type`].
///
///
pub fn resolve_type_parameter_type(to_resolve: Type, instance: &BaseTypeInstance) -> Type {
    match to_resolve {
        Type::Array { size, inner } => Type::Array {
            size,
            inner: Box::new(resolve_type_parameter_type(*inner, &instance)),
        },

        Type::Reference { mutable, inner } => Type::Reference {
            mutable,
            inner: Box::new(resolve_type_parameter_type(*inner, instance)),
        },

        Type::Base(_) => to_resolve,
        Type::TypeParameter { name: _, param_ind } => instance.type_parameters[param_ind].clone(),
    }
}

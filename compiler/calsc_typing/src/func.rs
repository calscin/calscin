//! Declaration for functions

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_utils::hash::HashedString;

use crate::tree::Type;

/// Represents a signature of a function
pub type TypeSignature = (Vec<Type>, Option<Type>);

/// Represents a function inside of the typing system
#[derive(Clone)]
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

/// Defines a type that can be affected by a `decl` block / have functions
pub trait DeclBlockAffectedType {
    /// Adds a [`TypedFunction`] inside of the type corresponding to the given name.
    ///
    /// # Errors
    /// Will return an error if the function is already present inside of the type
    fn add_function<K: DiagnosticSource>(
        &mut self,
        name: HashedString,
        func: TypedFunction,
        source: &K,
    ) -> DiagPossible;

    /// Checks if the given type has a the given function with the matching signature.
    ///
    /// **Warn: This exactly checks the signature and doesn't handle type parameters yet**
    ///
    /// We do not need a get function since the stored functions should only be the [`TypedFunction`]
    ///
    fn has_function(&self, name: HashedString, signature: TypeSignature) -> bool;
}

//! Definitions for global context values

use std::fmt::{Debug, Display};

use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource, diags::errors::build_expected_error,
};
use calsc_typing::{base::BaseType, tree::Type};

/// An entry / value inside of the global context.
/// This shouldn't be clonable due to the inner data modification not being able to be synced
pub enum GlobalContextValue {
    /// Represents a type-based entry
    Type(BaseType),

    /// Represents a type alias
    TypeAlias(Type),
}

impl GlobalContextValue {
    /// Creates a new [`GlobalContextValue`] from the
    pub fn new_type(inst: BaseType) -> Self {
        Self::Type(inst)
    }

    /// Converts the [`GlobalContextValue`] into a type entry and returns the [`BaseType`] associated with it.
    ///
    /// # Errors
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a type
    ///
    pub fn as_type<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<BaseType> {
        match self {
            Self::Type(inst) => Ok(inst.clone()),

            _ => return Err(build_expected_error(&"type".to_string(), self, origin).into()),
        }
    }

    /// Converts the [`GlobalContextValue`] into a type alias entry and returns the [`Type`] associated with it.
    ///
    /// # Errors
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a type alias
    ///
    pub fn as_type_alias<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<Type> {
        match self {
            Self::TypeAlias(ty) => Ok(ty.clone()),

            _ => return Err(build_expected_error(&"type alias".to_string(), self, origin).into()),
        }
    }

    /// Mutates the [`GlobalContextValue`] into a type entry and modifies the currently held [`BaseType`] according to the mutation function
    ///
    /// Mutation should be used to modify inner objects, not replace them
    ///
    /// # Erorrs
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a type
    ///
    pub fn mutate_type<K: DiagnosticSource, F>(&mut self, func: F, origin: &K) -> DiagPossible
    where
        F: FnOnce(&mut BaseType),
    {
        match self {
            Self::Type(inst) => {
                func(inst);

                Ok(())
            }

            _ => Err(build_expected_error(&"type".to_string(), self, origin).into()),
        }
    }

    /// Mutates the [`GlobalContextValue`] into a type alias entry and modifies the currently held [`Type`] according to the mutation function
    ///
    /// Mutation should be used to modify inner objects, not replace them
    ///
    /// # Erorrs
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a type alias
    ///
    pub fn mutate_type_alias<K: DiagnosticSource, F>(&mut self, func: F, origin: &K) -> DiagPossible
    where
        F: FnOnce(&mut Type),
    {
        match self {
            Self::TypeAlias(inst) => {
                func(inst);

                Ok(())
            }

            _ => Err(build_expected_error(&"type alias".to_string(), self, origin).into()),
        }
    }

    /// Checks whether the entry is a type or not
    pub fn is_type(&self) -> bool {
        match self {
            Self::Type(_) => true,

            _ => false,
        }
    }

    /// Checks whether the entry is a type alias or not
    pub fn is_type_alias(&self) -> bool {
        match self {
            Self::TypeAlias(_) => true,

            _ => false,
        }
    }
}

impl Display for GlobalContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Type(_) => "type",
            Self::TypeAlias(_) => "type alias",
        };

        write!(f, "{}", s)
    }
}

impl Debug for GlobalContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Type(_) => "type",
            Self::TypeAlias(_) => "type alias",
        };

        write!(f, "{}", s)
    }
}

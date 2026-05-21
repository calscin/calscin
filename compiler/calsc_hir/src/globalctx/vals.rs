//! Definitions for global context values

use std::fmt::Display;

use calsc_diagnostics::{DiagResult, DiagnosticSource, diags::errors::build_expected_error};
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
    pub fn as_type<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<BaseType> {
        match self {
            Self::Type(inst) => Ok(inst.clone()),

            _ => return Err(build_expected_error(&"type".to_string(), self, origin).into()),
        }
    }

    pub fn as_type_alias<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<Type> {
        match self {
            Self::TypeAlias(ty) => Ok(ty.clone()),

            _ => return Err(build_expected_error(&"type alias".to_string(), self, origin).into()),
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

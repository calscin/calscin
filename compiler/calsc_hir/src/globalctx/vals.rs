//! Definitions for global context values

use std::fmt::{Debug, Display};

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_expected_error, build_unexpected_error},
};
use calsc_typing::{base::BaseType, tree::Type};

use crate::{funcs::HIRFunction, types::safely_make_type_instance};

/// An entry / value inside of the global context.
/// This shouldn't be clonable due to the inner data modification not being able to be synced
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum GlobalContextValue {
    /// Represents a type-based entry
    Type(BaseType),

    /// Represents a type alias
    TypeAlias(Type),

    Function(HIRFunction),
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

    /// Gets a type from the given [`GlobalContextValue`].
    /// This function works both with types and type aliases and will craft a new empty [`Type::Base`] instance for raw types.
    pub fn craft_type<K: DiagnosticSource>(
        &self,
        origin: &K,
        size_specifiers: Vec<usize>,
        type_parameters: Vec<Type>,
    ) -> DiagResult<Type> {
        match self {
            Self::TypeAlias(ty) => {
                if size_specifiers.is_empty() && type_parameters.is_empty() {
                    Ok(ty.clone())
                } else {
                    Err(build_unexpected_error(
                        &"additional type parameters or size specifiers".to_string(),
                        origin,
                    )
                    .into())
                }
            }
            Self::Type(ty) => Ok(Type::Base(safely_make_type_instance(
                ty.clone(),
                size_specifiers,
                type_parameters,
                origin,
            )?)),

            _ => return Err(build_expected_error(&"type".to_string(), self, origin).into()),
        }
    }

    /// Converts the [`GlobalContextValue`] into a function entry and returns the [`HIRFunction`] reference associated with it.
    ///
    /// # Errors
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a function
    ///
    pub fn as_function<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<&HIRFunction> {
        match self {
            Self::Function(func) => Ok(func),

            _ => return Err(build_expected_error(&"function".to_string(), self, origin).into()),
        }
    }

    /// Mutates the [`GlobalContextValue`] into a type entry and modifies the currently held [`BaseType`] according to the mutation function
    ///
    /// Mutation should be used to modify inner objects, not replace them
    ///
    /// # Erorrs
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a type
    ///
    pub fn mutate_type<K: DiagnosticSource, F, R>(&mut self, func: F, origin: &K) -> DiagResult<R>
    where
        F: FnOnce(&mut BaseType) -> R,
    {
        match self {
            Self::Type(inst) => Ok(func(inst)),

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
    pub fn mutate_type_alias<K: DiagnosticSource, F, R>(
        &mut self,
        func: F,
        origin: &K,
    ) -> DiagResult<R>
    where
        F: FnOnce(&mut Type) -> R,
    {
        match self {
            Self::TypeAlias(inst) => Ok(func(inst)),

            _ => Err(build_expected_error(&"type alias".to_string(), self, origin).into()),
        }
    }

    /// Mutates the [`GlobalContextValue`] into a function entry and modifies the currently held [`HIRFunction`] according to the mutation function
    ///
    /// Mutation should be used to modify inner objects, not replace them
    ///
    /// # Erorrs
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a function
    ///
    pub fn mutate_function<K: DiagnosticSource, F, R>(
        &mut self,
        func: F,
        origin: &K,
    ) -> DiagResult<R>
    where
        F: FnOnce(&mut HIRFunction) -> R,
    {
        match self {
            Self::Function(f) => Ok(func(f)),

            _ => Err(build_expected_error(&"function".to_string(), self, origin).into()),
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

    /// Checks whether the entry is a function or not
    pub fn is_function(&self) -> bool {
        match self {
            Self::Function(_) => true,

            _ => false,
        }
    }
}

impl Display for GlobalContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Type(_) => "type",
            Self::TypeAlias(_) => "type alias",
            Self::Function(_) => "function",
        };

        write!(f, "{}", s)
    }
}

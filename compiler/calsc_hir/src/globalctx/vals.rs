//! Definitions for global context values

use std::fmt::{Debug, Display};

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_expected_entry_type, build_unexpected_type_alias_additional_parameters},
};

use calsc_modules::path::ModulePath;
use calsc_typing_v2::{
    ctx::TypeCtx,
    types::{SizeParameter, TypeKind, primitive::PrimitiveType},
};

use crate::{funcs::HIRFunction, globalctx::key::GlobalContextKey};

/// An entry / value inside of the global context.
/// This shouldn't be clonable due to the inner data modification not being able to be synced
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)] // For MIR
pub enum GlobalContextValue {
    /// Represents a type-based entry
    Type(PrimitiveType),

    /// Represents a type alias
    TypeAlias(TypeKind),

    AnotherReference(GlobalContextKey),

    /// Represents an imported module
    Module(ModulePath),

    Function(HIRFunction),
}

impl GlobalContextValue {
    /// Creates a new [`GlobalContextValue`] from the
    pub fn new_type(inst: PrimitiveType) -> Self {
        Self::Type(inst)
    }

    /// Converts the [`GlobalContextValue`] into a type entry and returns the [`BaseType`] associated with it.
    ///
    /// # Errors
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a type
    ///
    pub fn as_type<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<PrimitiveType> {
        match self {
            Self::Type(inst) => Ok(inst.clone()),

            _ => return Err(build_expected_entry_type(&"type".to_string(), self, origin).into()),
        }
    }

    /// Converts the [`GlobalContextValue`] into a type alias entry and returns the [`Type`] associated with it.
    ///
    /// # Errors
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a type alias
    ///
    pub fn as_type_alias<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<TypeKind> {
        match self {
            Self::TypeAlias(ty) => Ok(ty.clone()),

            _ => {
                return Err(
                    build_expected_entry_type(&"type alias".to_string(), self, origin).into(),
                );
            }
        }
    }

    /// Converts the [`GlobalContextValue`] into a module import entry and returns the [`ModulePath`] associated with it.
    ///
    /// # Errors
    /// This function will error on the given [`DiagnosticSource`] if the entry is not a module import
    ///
    pub fn as_module<K: DiagnosticSource>(&self, origin: &K) -> DiagResult<ModulePath> {
        match self {
            Self::Module(path) => Ok(path.clone()),

            _ => {
                return Err(build_expected_entry_type(&"path".to_string(), self, origin).into());
            }
        }
    }

    /// Gets a type from the given [`GlobalContextValue`].
    /// This function works both with types and type aliases and will craft a new empty [`Type::Base`] instance for raw types.
    pub fn craft_type<K: DiagnosticSource>(
        &self,
        origin: &K,
        ctx: &TypeCtx,
        size_parameter: SizeParameter,
    ) -> DiagResult<TypeKind> {
        match self {
            Self::TypeAlias(ty) => {
                if !size_parameter.is_active() {
                    Ok(ty.clone())
                } else {
                    Err(build_unexpected_type_alias_additional_parameters(origin).into())
                }
            }

            Self::Type(ty) => TypeKind::new_primitive(ty.clone(), size_parameter, ctx, origin),

            _ => return Err(build_expected_entry_type(&"type".to_string(), self, origin).into()),
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

            _ => {
                return Err(
                    build_expected_entry_type(&"function".to_string(), self, origin).into(),
                );
            }
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
        F: FnOnce(&mut PrimitiveType) -> R,
    {
        match self {
            Self::Type(inst) => Ok(func(inst)),

            _ => Err(build_expected_entry_type(&"type".to_string(), self, origin).into()),
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
        F: FnOnce(&mut TypeKind) -> R,
    {
        match self {
            Self::TypeAlias(inst) => Ok(func(inst)),

            _ => Err(build_expected_entry_type(&"type alias".to_string(), self, origin).into()),
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

            _ => Err(build_expected_entry_type(&"function".to_string(), self, origin).into()),
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

    pub fn is_module(&self) -> bool {
        match self {
            Self::Module(_) => true,

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

    pub fn is_reference(&self) -> bool {
        match self {
            Self::AnotherReference(_) => true,
            _ => false,
        }
    }
}

impl Display for GlobalContextValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Type(_) => "type",
            Self::Module(_) => "module",
            Self::TypeAlias(_) => "type alias",
            Self::Function(_) => "function",
            Self::AnotherReference(_) => "reference to another entry",
        };

        write!(f, "{}", s)
    }
}

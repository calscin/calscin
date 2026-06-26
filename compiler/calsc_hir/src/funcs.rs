//! Declarations for HIR functions

use calsc_typing::{params::TypeParameterId, types::TypeKind};
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::{globalctx::key::GlobalContextKey, localctx::LocalContext};

/// Represents a function in the HIR stage
///
/// Functions stored can have 3 types:
/// - **Extern**: functions that do not have a implementation node nor a local context
/// - **Stage 1**: functions that have a local context but no implementation yet
/// - **Stage 2**: functions that have both a local context and an implementation
///
/// The function seeking goes into two phases:
/// - **Stage 1**: Seeking of every function without parsing the body in order to know every function. Creates functions of type stage 1
///
/// - **Stage 2**: Convertion of function bodies into HIR vartiants using the seeked functions. Modifies stage 1 type functions into stage 2 functions.
///
/// This system allows for recursion and out of order function calling.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)] // For MIR
pub struct HIRFunction {
    /// Represents the key related to the function
    pub name: GlobalContextKey,

    /// The function name used during the Remir lowering
    pub actual_function_name: HashedString,

    /// The local context
    /// Is present in stage 1 and stage 2 functions
    pub local_context: Option<LocalContext>,

    /// The return type of the function
    pub return_type: TypeKind,

    /// The type parameters owned by the function (aka the type parameters of the function).
    pub type_parameters: Vec<TypeParameterId>,

    /// The arguments of the function
    pub arguments: Vec<(HashedString, TypeKind)>,

    /// The implementation node
    /// Is present only in stage 2 functions
    pub impl_node: Option<ArenaHandle>,

    /// The triple dot position for extern functions
    pub triple_dot_position: Option<usize>,

    pub is_main_function: bool,
}

impl HIRFunction {
    pub fn new_extern(
        name: GlobalContextKey,
        return_type: TypeKind,
        arguments: Vec<(HashedString, TypeKind)>,
        triple_dot_position: Option<usize>,
        is_main_function: bool,
    ) -> Self {
        Self {
            name: name.clone(),
            local_context: None,
            return_type,
            arguments,
            impl_node: None,
            actual_function_name: name.name.clone(),
            triple_dot_position,
            is_main_function,
            type_parameters: vec![],
        }
    }

    pub fn new_imported(
        name: GlobalContextKey,
        return_type: TypeKind,
        arguments: Vec<(HashedString, TypeKind)>,
        is_main_function: bool,
    ) -> Self {
        Self {
            name: name.clone(),
            local_context: None,
            return_type,
            arguments,
            impl_node: None,
            actual_function_name: format!("{}", name).into(),
            triple_dot_position: None,
            is_main_function,
            type_parameters: vec![],
        }
    }

    pub fn new_stage_1(
        name: GlobalContextKey,
        local_ctx: LocalContext,
        return_type: TypeKind,
        arguments: Vec<(HashedString, TypeKind)>,
        is_main_function: bool,
    ) -> Self {
        Self {
            name: name.clone(),
            local_context: Some(local_ctx),
            return_type,
            arguments,
            impl_node: None,
            triple_dot_position: None,
            is_main_function,
            actual_function_name: format!("{}", name).into(),
            type_parameters: vec![],
        }
    }

    /// Transforms a stage 1 function into a stage 2 function by consuming the implementation node reference.
    ///
    /// # Panics
    /// This function will panic if this is not a stage 1 function
    ///
    pub fn transform_stage_2(&mut self, impl_node: ArenaHandle) {
        if self.local_context.is_none() {
            panic!("This is not a stage 1 function");
        }

        self.impl_node = Some(impl_node);
    }
}

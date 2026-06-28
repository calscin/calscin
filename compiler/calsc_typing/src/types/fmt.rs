use std::fmt::Display;

use calsc_utils::{DisplayWith, display_with_list, display_with_to_string};

use crate::{
    allocs::STRUCT_CONTAINER_ALLOC,
    ctx::TypeCtx,
    types::{MutationState, SizeParameter, TypeKind, primitive::PrimitiveType},
};

impl DisplayWith<&TypeCtx> for PrimitiveType {
    fn fmt(&self, k: &TypeCtx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(signed) => write!(f, "{}", if *signed { "s" } else { "u" }),
            Self::Float => write!(f, "f"),
            Self::Boolean => write!(f, "bool"),
            Self::Str => write!(f, "str"),
            Self::Size => write!(f, "size"),
            Self::Struct(container) => STRUCT_CONTAINER_ALLOC.with(|ff| {
                let arena_ref = ff.borrow().get(container);

                write!(f, "{}::{}", arena_ref.module, arena_ref.name)
            }),

            Self::Function(func) => {
                let arena_ref = k.typed_function_arena.get(func);

                write!(
                    f,
                    "func ({}) -> {}",
                    display_with_list(&arena_ref.arguments, k),
                    display_with_to_string(&arena_ref.return_type, k)
                )
            }

            Self::TypeParameter(param) => write!(f, "{}", param.1),
        }
    }
}

impl Display for MutationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.0 {
            return Ok(());
        }

        write!(f, " mut")
    }
}

impl Display for SizeParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_active() {
            return Ok(());
        }

        write!(f, ".{}", self.0)
    }
}

impl DisplayWith<&TypeCtx> for TypeKind {
    fn fmt(&self, k: &TypeCtx, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reference(mutation, handle) => {
                k.type_kind_arena.get(handle).fmt(k, f)?;

                write!(f, "&{}", mutation)
            }

            Self::Pointer(mutation, handle) => {
                k.type_kind_arena.get(handle).fmt(k, f)?;

                write!(f, "*{}", mutation)
            }

            Self::Array(size, handle) => {
                k.type_kind_arena.get(handle).fmt(k, f)?;

                write!(f, "[{}]", size)
            }

            Self::Segment(handle) => {
                k.type_kind_arena.get(handle).fmt(k, f)?;

                write!(f, "[]")
            }

            Self::Primitive(primitive) => {
                write!(
                    f,
                    "{}{}",
                    &display_with_to_string(&primitive.ty, k),
                    primitive.size
                )
            }

            Self::Void => write!(f, "void"),
        }
    }
}

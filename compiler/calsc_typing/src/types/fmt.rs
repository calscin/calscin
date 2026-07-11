use std::fmt::Display;

use calsc_utils::{DisplayWith, display_with_list, display_with_to_string};

use crate::{
    TypingInterner,
    types::{MutationState, SizeParameter, TypeKind, primitive::PrimitiveType},
};

impl DisplayWith<&TypingInterner> for PrimitiveType {
    fn fmt(&self, k: &TypingInterner, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(signed) => write!(f, "{}", if *signed { "s" } else { "u" }),
            Self::Float => write!(f, "f"),
            Self::Boolean => write!(f, "bool"),
            Self::Str => write!(f, "str"),
            Self::Size => write!(f, "size"),
            Self::Struct(container) => {
                let handle = k.struct_container_arena.get(container);

                write!(f, "{}::{}", handle.module, handle.name)
            }

            Self::Function(func) => {
                let arena_ref = k.func_arena.get(func);

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

impl DisplayWith<&TypingInterner> for TypeKind {
    fn fmt(&self, k: &TypingInterner, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

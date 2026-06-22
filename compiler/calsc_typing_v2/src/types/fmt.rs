use std::fmt::Display;

use calsc_utils::DisplayWith;

use crate::{
    allocs::TypeKindArena,
    types::{MutationState, SizeParameter, TypeKind, primitive::PrimitiveType},
};

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveType::Int(signed) => write!(f, "{}", if *signed { "s" } else { "u" }),
            PrimitiveType::Float => write!(f, "f"),
            PrimitiveType::Str => write!(f, "str"),
            PrimitiveType::Boolean => write!(f, "bool"),
            PrimitiveType::Size => write!(f, "size"),
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

impl DisplayWith<&TypeKindArena> for TypeKind {
    fn fmt(&self, k: &TypeKindArena, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reference(mutation, handle) => {
                k.get(handle).fmt(k, f)?;

                write!(f, "&{}", mutation)
            }

            Self::Pointer(mutation, handle) => {
                k.get(handle).fmt(k, f)?;

                write!(f, "*{}", mutation)
            }

            Self::Array(size, handle) => {
                k.get(handle).fmt(k, f)?;

                write!(f, "[{}]", size)
            }

            Self::Segment(handle) => {
                k.get(handle).fmt(k, f)?;

                write!(f, "[]")
            }

            Self::Primitive(primitive, size_param) => write!(f, "{}{}", primitive, size_param),
        }
    }
}

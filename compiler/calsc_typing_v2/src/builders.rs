//! Common builders for frequently used types

use crate::types::{SizeParameter, TypeKind, primitive::PrimitiveType};

impl TypeKind {
    pub fn make_int_type(signed: bool, size: usize) -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Int(signed), SizeParameter(size))
    }

    pub fn make_float_type(size: usize) -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Float, SizeParameter(size))
    }

    pub fn make_bool_type() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Boolean, SizeParameter(0))
    }

    pub fn make_str_type() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Str, SizeParameter(0))
    }

    pub fn make_size_type() -> TypeKind {
        TypeKind::Primitive(PrimitiveType::Size, SizeParameter(0))
    }
}

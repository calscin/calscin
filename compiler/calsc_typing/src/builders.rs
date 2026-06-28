//! Common builders for frequently used types

use std::collections::HashMap;

use crate::types::{HeldPrimitive, SizeParameter, TypeKind, primitive::PrimitiveType};

impl TypeKind {
    pub fn make_int_type(signed: bool, size: usize) -> TypeKind {
        TypeKind::Primitive(HeldPrimitive {
            ty: PrimitiveType::Int(signed),
            size: SizeParameter(size),
            type_parameters: HashMap::new(),
        })
    }

    pub fn make_float_type(size: usize) -> TypeKind {
        TypeKind::Primitive(HeldPrimitive {
            ty: PrimitiveType::Float,
            size: SizeParameter(size),
            type_parameters: HashMap::new(),
        })
    }

    pub fn make_bool_type() -> TypeKind {
        TypeKind::Primitive(HeldPrimitive {
            ty: PrimitiveType::Boolean,
            size: SizeParameter(0),
            type_parameters: HashMap::new(),
        })
    }

    pub fn make_str_type() -> TypeKind {
        TypeKind::Primitive(HeldPrimitive {
            ty: PrimitiveType::Str,
            size: SizeParameter(0),
            type_parameters: HashMap::new(),
        })
    }

    pub fn make_size_type() -> TypeKind {
        TypeKind::Primitive(HeldPrimitive {
            ty: PrimitiveType::Size,
            size: SizeParameter(0),
            type_parameters: HashMap::new(),
        })
    }
}

use calsc_utils::hash::HashedString;

use crate::{
    TypingInterner,
    types::{TypeKind, primitive::PrimitiveType, structs::StructContainer},
};

pub trait TypeParameteredType {
    fn get_type_params(&self, interner: &TypingInterner) -> Vec<HashedString>;
    fn has_type_param(&self, name: &HashedString, interner: &TypingInterner) -> bool;
}

impl TypeParameteredType for StructContainer {
    fn get_type_params(&self, _interner: &TypingInterner) -> Vec<HashedString> {
        self.type_parameters.clone()
    }

    fn has_type_param(&self, name: &HashedString, _interner: &TypingInterner) -> bool {
        self.type_parameters.contains(name)
    }
}

impl TypeParameteredType for PrimitiveType {
    fn get_type_params(&self, interner: &TypingInterner) -> Vec<HashedString> {
        match self {
            PrimitiveType::Struct(container) => interner
                .struct_container_arena
                .get(container)
                .get_type_params(interner),

            _ => vec![],
        }
    }

    fn has_type_param(&self, name: &HashedString, interner: &TypingInterner) -> bool {
        match self {
            Self::Struct(container) => interner
                .struct_container_arena
                .get(container)
                .has_type_param(name, interner),

            _ => false,
        }
    }
}

impl TypeParameteredType for TypeKind {
    fn get_type_params(&self, interner: &TypingInterner) -> Vec<HashedString> {
        match self {
            Self::Primitive(primitive) => primitive.ty.get_type_params(interner),
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_type_params(interner),
            Self::Pointer(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_type_params(interner),
            Self::Array(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_type_params(interner),
            Self::Segment(inner) => interner
                .type_kind_arena
                .get(inner)
                .get_type_params(interner),
            Self::Void => vec![],
        }
    }

    fn has_type_param(&self, name: &HashedString, interner: &TypingInterner) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.ty.has_type_param(name, interner),
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .has_type_param(name, interner),
            Self::Pointer(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .has_type_param(name, interner),
            Self::Array(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .has_type_param(name, interner),
            Self::Segment(inner) => interner
                .type_kind_arena
                .get(inner)
                .has_type_param(name, interner),
            Self::Void => false,
        }
    }
}

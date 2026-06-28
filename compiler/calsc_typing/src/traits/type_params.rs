use calsc_utils::hash::HashedString;

use crate::{
    allocs::STRUCT_CONTAINER_ALLOC,
    ctx::TypeCtx,
    types::{TypeKind, primitive::PrimitiveType, structs::StructContainer},
};

pub trait TypeParameteredType {
    fn get_type_params(&self, ctx: &TypeCtx) -> Vec<HashedString>;
    fn has_type_param(&self, name: &HashedString, ctx: &TypeCtx) -> bool;
}

impl TypeParameteredType for StructContainer {
    fn get_type_params(&self, _ctx: &TypeCtx) -> Vec<HashedString> {
        self.type_parameters.clone()
    }

    fn has_type_param(&self, name: &HashedString, _ctx: &TypeCtx) -> bool {
        self.type_parameters.contains(name)
    }
}

impl TypeParameteredType for PrimitiveType {
    fn get_type_params(&self, ctx: &TypeCtx) -> Vec<HashedString> {
        match self {
            PrimitiveType::Struct(container) => {
                STRUCT_CONTAINER_ALLOC.with(|f| f.borrow().get(&container).get_type_params(ctx))
            }

            _ => vec![],
        }
    }

    fn has_type_param(&self, name: &HashedString, ctx: &TypeCtx) -> bool {
        match self {
            Self::Struct(container) => STRUCT_CONTAINER_ALLOC
                .with(|f| f.borrow().get(&container).has_type_param(name, ctx)),

            _ => false,
        }
    }
}

impl TypeParameteredType for TypeKind {
    fn get_type_params(&self, ctx: &TypeCtx) -> Vec<HashedString> {
        match self {
            Self::Primitive(primitive) => primitive.ty.get_type_params(ctx),
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).get_type_params(ctx),
            Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner).get_type_params(ctx),
            Self::Array(_, inner) => ctx.type_kind_arena.get(inner).get_type_params(ctx),
            Self::Segment(inner) => ctx.type_kind_arena.get(inner).get_type_params(ctx),
            Self::Void => vec![],
        }
    }

    fn has_type_param(&self, name: &HashedString, ctx: &TypeCtx) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.ty.has_type_param(name, ctx),
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).has_type_param(name, ctx),
            Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner).has_type_param(name, ctx),
            Self::Array(_, inner) => ctx.type_kind_arena.get(inner).has_type_param(name, ctx),
            Self::Segment(inner) => ctx.type_kind_arena.get(inner).has_type_param(name, ctx),
            Self::Void => false,
        }
    }
}

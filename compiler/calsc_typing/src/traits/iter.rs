//! Traits related to iteration

use crate::{ctx::TypeCtx, types::TypeKind};

pub trait IterableType {
    fn is_iterable(&self, ty: &TypeKind, ctx: &TypeCtx) -> bool;

    fn is_iterable_at_all(&self, ctx: &TypeCtx) -> bool;

    fn get_iterator_type(&self, ctx: &TypeCtx) -> TypeKind;

    fn get_iterator_output_type(&self, ctx: &TypeCtx) -> TypeKind;
}

impl IterableType for TypeKind {
    fn is_iterable(&self, ty: &TypeKind, ctx: &TypeCtx) -> bool {
        match self {
            Self::Array(_, _) => ty == &self.get_iterator_type(ctx),
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).is_iterable(ty, ctx),

            _ => false,
        }
    }

    fn is_iterable_at_all(&self, ctx: &TypeCtx) -> bool {
        match self {
            Self::Array(_, _) => true,
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).is_iterable_at_all(ctx),

            _ => false,
        }
    }

    fn get_iterator_type(&self, ctx: &TypeCtx) -> TypeKind {
        match self {
            Self::Array(_, _) => TypeKind::make_size_type(),
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).get_iterator_type(ctx),

            _ => panic!("This type is not iterable"),
        }
    }

    fn get_iterator_output_type(&self, ctx: &TypeCtx) -> TypeKind {
        match self {
            Self::Array(_, inner) => ctx.type_kind_arena.get(inner).clone(),
            Self::Reference(_, inner) => {
                ctx.type_kind_arena.get(inner).get_iterator_output_type(ctx)
            }

            _ => panic!("This type is not iterable"),
        }
    }
}

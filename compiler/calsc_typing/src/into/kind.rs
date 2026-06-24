//! Type convertions for type kind

use crate::{
    ctx::TypeCtx,
    into::{TypeCasting, TypeTransmutation},
    types::{TypeKind, primitive::PrimitiveType},
};

impl TypeTransmutation for TypeKind {
    fn can_transmute(&self, into: &Self, ctx: &TypeCtx) -> bool {
        match (self, into) {
            (Self::Pointer(mutable, _), Self::Pointer(into_mutable, _)) => {
                mutable == into_mutable || !into_mutable.0
            }

            (Self::Reference(mutable, _), Self::Pointer(into_mutable, _)) => {
                mutable == into_mutable || !into_mutable.0
            }

            (Self::Primitive(primitive, _), Self::Pointer(_, _)) => {
                primitive == &PrimitiveType::Size
            }

            (Self::Primitive(primitive, size), Self::Primitive(into_primitive, into_size)) => {
                size.is_active() == into_size.is_active()
                    && into_size.0 >= size.0
                    && primitive.can_transmute(into_primitive, ctx)
            }

            _ => false,
        }
    }

    fn can_transmute_weakly(&self, into: &Self, ctx: &TypeCtx) -> bool {
        if self.can_transmute(into, ctx) {
            return true;
        }

        match (self, into) {
            (Self::Primitive(primitive, _), Self::Primitive(into_primitive, _)) => {
                primitive.can_transmute_weakly(into_primitive, ctx)
            }

            _ => false,
        }
    }
}

impl TypeCasting for TypeKind {
    fn can_cast(&self, into: &Self, ctx: &TypeCtx) -> bool {
        if self.can_transmute(into, ctx) {
            return true; // Allow every transmutation to be done with casts
        }

        match (self, into) {
            (Self::Pointer(mutable, inner), Self::Reference(into_mutable, into_inner)) => {
                mutable == into_mutable && inner == into_inner
            }

            (Self::Pointer(mutable, _), Self::Pointer(_, _)) => !mutable.0,

            (Self::Primitive(primitive, _), Self::Primitive(into_primitive, _)) => {
                primitive.can_cast(into_primitive, ctx)
            }

            _ => false,
        }
    }
}

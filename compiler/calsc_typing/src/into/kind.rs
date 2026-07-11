//! Type convertions for type kind

use crate::{
    TypingInterner,
    into::{TypeCasting, TypeTransmutation},
    types::{TypeKind, primitive::PrimitiveType},
};

impl TypeTransmutation for TypeKind {
    fn can_transmute(&self, into: &Self, interner: &TypingInterner) -> bool {
        match (self, into) {
            (Self::Pointer(mutable, _), Self::Pointer(into_mutable, _)) => {
                mutable == into_mutable || !into_mutable.0
            }

            (Self::Reference(mutable, _), Self::Pointer(into_mutable, _)) => {
                mutable == into_mutable || !into_mutable.0
            }

            (Self::Primitive(primitive), Self::Pointer(_, _)) => {
                &primitive.ty == &PrimitiveType::Size
            }

            (Self::Primitive(primitive), Self::Primitive(into)) => {
                primitive.size.is_active() == into.size.is_active()
                    && into.size.0 >= primitive.size.0
                    && primitive.ty.can_transmute(&primitive.ty, interner)
            }

            _ => false,
        }
    }

    fn can_transmute_weakly(&self, into: &Self, interner: &TypingInterner) -> bool {
        if self.can_transmute(into, interner) {
            return true;
        }

        match (self, into) {
            (Self::Primitive(primitive), Self::Primitive(into_primitive)) => primitive
                .ty
                .can_transmute_weakly(&into_primitive.ty, interner),

            _ => false,
        }
    }
}

impl TypeCasting for TypeKind {
    fn can_cast(&self, into: &Self, interner: &TypingInterner) -> bool {
        if self.can_transmute(into, interner) {
            return true; // Allow every transmutation to be done with casts
        }

        match (self, into) {
            (Self::Pointer(mutable, inner), Self::Reference(into_mutable, into_inner)) => {
                mutable == into_mutable
                    && interner.type_kind_arena.get(inner)
                        == interner.type_kind_arena.get(into_inner)
            }

            (Self::Pointer(mutable, _), Self::Pointer(_, _)) => !mutable.0,

            (Self::Primitive(primitive), Self::Primitive(into_primitive)) => {
                primitive.ty.can_cast(&into_primitive.ty, interner)
            }

            _ => false,
        }
    }
}

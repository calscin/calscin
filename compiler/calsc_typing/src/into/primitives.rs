//! Type convertions for primitives

use crate::{
    TypingInterner,
    into::{TypeCasting, TypeTransmutation},
    types::primitive::PrimitiveType,
};

impl TypeTransmutation for PrimitiveType {
    fn can_transmute(&self, into: &Self, _interner: &TypingInterner) -> bool {
        if self == into {
            return true;
        }

        match (self, into) {
            (PrimitiveType::Int(signed), PrimitiveType::Int(_)) => !signed, // Allow unsigned -> signed convertion
            (PrimitiveType::Int(signed), PrimitiveType::Size) => !signed,

            _ => false,
        }
    }

    fn can_transmute_weakly(&self, into: &Self, interner: &TypingInterner) -> bool {
        if self.can_transmute(into, interner) {
            return true;
        }

        match (self, into) {
            (PrimitiveType::Int(signed), PrimitiveType::Size) => !signed, // Allow unsigned int -> size

            _ => false,
        }
    }
}

impl TypeCasting for PrimitiveType {
    fn can_cast(&self, into: &Self, _interner: &TypingInterner) -> bool {
        if self == into {
            return true;
        }

        match (self, into) {
            (PrimitiveType::Int(_), PrimitiveType::Float) => true,
            (PrimitiveType::Float, PrimitiveType::Int(_)) => true,

            (PrimitiveType::Int(_), PrimitiveType::Int(_)) => true,
            (PrimitiveType::Size, PrimitiveType::Int(_)) => true,
            (PrimitiveType::Int(_), PrimitiveType::Size) => true,

            _ => false,
        }
    }
}

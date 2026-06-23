//! Type convertions for primitives

use crate::{
    into::{TypeCasting, TypeTransmutation},
    types::primitive::PrimitiveType,
};

impl TypeTransmutation for PrimitiveType {
    fn can_transmute(&self, into: &Self) -> bool {
        if self == into {
            return true;
        }

        match (self, into) {
            (PrimitiveType::Int(signed), PrimitiveType::Int(_)) => !signed, // Allow unsigned -> signed convertion

            _ => false,
        }
    }

    fn can_transmute_weakly(&self, into: &Self) -> bool {
        if self.can_transmute(into) {
            return true;
        }

        match (self, into) {
            (PrimitiveType::Int(signed), PrimitiveType::Size) => !signed, // Allow unsigned int -> size

            _ => false,
        }
    }
}

impl TypeCasting for PrimitiveType {
    fn can_cast(&self, into: &Self) -> bool {
        if self == into {
            return true;
        }

        match (self, into) {
            (PrimitiveType::Int(_), PrimitiveType::Float) => true,
            (PrimitiveType::Float, PrimitiveType::Int(_)) => true,

            (PrimitiveType::Int(_), PrimitiveType::Int(_)) => true,

            _ => false,
        }
    }
}

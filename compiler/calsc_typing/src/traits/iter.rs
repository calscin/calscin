//! Traits related to iteration

use crate::{TypingInterner, types::TypeKind};

pub trait IterableType {
    fn is_iterable(&self, ty: &TypeKind, interner: &TypingInterner) -> bool;

    fn is_iterable_at_all(&self, interner: &TypingInterner) -> bool;

    fn get_iterator_type(&self, interner: &TypingInterner) -> TypeKind;

    fn get_iterator_output_type(&self, interner: &TypingInterner) -> TypeKind;
}

impl IterableType for TypeKind {
    fn is_iterable(&self, ty: &TypeKind, interner: &TypingInterner) -> bool {
        match self {
            Self::Array(_, _) => ty == &self.get_iterator_type(interner),
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .is_iterable(ty, interner),

            _ => false,
        }
    }

    fn is_iterable_at_all(&self, interner: &TypingInterner) -> bool {
        match self {
            Self::Array(_, _) => true,
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .is_iterable_at_all(interner),

            _ => false,
        }
    }

    fn get_iterator_type(&self, interner: &TypingInterner) -> TypeKind {
        match self {
            Self::Array(_, _) => TypeKind::make_size_type(),
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_iterator_type(interner),

            _ => panic!("This type is not iterable"),
        }
    }

    fn get_iterator_output_type(&self, interner: &TypingInterner) -> TypeKind {
        match self {
            Self::Array(_, inner) => interner.type_kind_arena.get(inner).clone(),
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_iterator_output_type(interner),

            _ => panic!("This type is not iterable"),
        }
    }
}

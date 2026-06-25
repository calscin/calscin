//! Type hints allow for type determination with a simple system.
//!
//! This system is experimental and may be used for later purposes

use crate::types::TypeKind;

pub enum TypeHint {
    /// A strong type hint. Represents a type hint that cannot be overriden.
    /// There can only be a single type hint per query.
    Strong(TypeKind),

    /// A weak type hint. Represents a type hint that can be overriden.
    /// There can be multiple weak hints per query.
    Weak(TypeKind),
}

pub struct TypeHintContainer {
    pub strong_hints: Vec<TypeHint>,
    pub weak_hints: Vec<TypeHint>,
}

impl TypeHintContainer {
    /// Creates a new [`TypeHintContainer`]
    pub fn new() -> Self {
        Self {
            strong_hints: vec![],
            weak_hints: vec![],
        }
    }

    /// Appends a new [`TypeHint`] inside of the [`TypeHintContainer`]
    pub fn append(&mut self, hint: TypeHint) {
        if hint.is_strong() {
            self.strong_hints.push(hint);
        } else {
            self.weak_hints.push(hint);
        }
    }
}

impl TypeHint {
    /// Checks if the type hint is a strong hint.
    pub fn is_strong(&self) -> bool {
        matches!(self, TypeHint::Strong(_))
    }

    /// Gets the type hint's type.
    pub fn get_type(&self) -> &TypeKind {
        match self {
            Self::Strong(ty) => ty,
            Self::Weak(ty) => ty,
        }
    }
}

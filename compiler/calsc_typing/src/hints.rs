//! Type hints allow for type determination with a simple system.

use crate::types::TypeKind;

pub enum TypeHint {
    /// A strong type hint. Represents a type hint that cannot be overriden.
    /// There can only be a single type hint per query.
    Strong(TypeKind),

    /// A weak type hint. Represents a type hint that can be overriden.
    /// There can be multiple weak hints per query.
    Weak(TypeKind),
}

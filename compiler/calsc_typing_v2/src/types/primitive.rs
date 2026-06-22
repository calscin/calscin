//! Definitions for primitive types. Primitive types are the root of types and represent the actual concrete type.

pub enum PrimitiveType {
    /// Represents an integer type with a given signed state.
    Int(bool),

    /// Represents a signed float type.
    Float,

    /// Represents a string type
    Str,

    /// Represents a boolean type
    Boolean,

    /// Represents a size type
    Size,
}

impl PrimitiveType {
    /// Checks whenther the [`PrimitiveType`] requires a size specifier to be created.
    pub fn requires_size_parameter(&self) -> bool {
        match self {
            Self::Int(_) => true,
            Self::Float => true,

            _ => false,
        }
    }
}

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

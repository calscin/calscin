//! Definitions for the kind of base type. A base type can also hold information such as functions and more

pub enum BaseTypeKind {
    /// An integer type that is possibly signed
    Integer { signed: bool },

    /// A floating type that is possibly signed
    Floating { signed: bool },

    /// A boolean type
    Boolean,
}

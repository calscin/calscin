//! Definitions for the kind of base type. A base type can also hold information such as functions and more

#[derive(PartialEq)] // Remove this and replace it with a custom implementation whenever structs are added
pub enum BaseTypeKind {
    /// An integer type that is possibly signed
    Integer { signed: bool },

    /// A floating type that is possibly signed
    Floating { signed: bool },

    /// A boolean type
    Boolean,
}

impl BaseTypeKind {
    /// Get the amount of required size parameters to create a base type.
    /// Is used by [`BaseType::new`][`crate::base::BaseType::new`]
    pub fn get_required_size_parameters(&self) -> usize {
        match self {
            Self::Integer { .. } | Self::Floating { .. } => 1,
            _ => 0,
        }
    }
}

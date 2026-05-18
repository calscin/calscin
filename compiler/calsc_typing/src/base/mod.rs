//! Definitions for base types. They are also named generics inside of the typing system.

use crate::{base::kind::BaseTypeKind, tree::Type};

pub mod kind;

pub struct BaseType {
    /// The kind of the base type
    pub kind: BaseTypeKind,

    /// The size specifiers of the type
    pub size_specifiers: Vec<usize>,

    /// The type parameters of the type
    pub type_parameters: Vec<Type>,
}

impl BaseType {
    /// Creates a new [`BaseType`] instance with the given kind and the given type and size specifiers.
    ///
    /// # Panics
    /// This function will panic if the amount ofsize specifiers aren't equal to the amount required.
    ///
    pub fn new(
        kind: BaseTypeKind,
        size_specifiers: Vec<usize>,
        type_parameters: Vec<Type>,
    ) -> Self {
        if size_specifiers.len() == kind.get_required_size_parameters() {
            Self {
                kind,
                size_specifiers,
                type_parameters,
            }
        } else {
            panic!(
                "Expected {} size parameters but got {} size parameters",
                kind.get_required_size_parameters(),
                size_specifiers.len()
            )
        }
    }
}

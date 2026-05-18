//! Definitions for base types. They are also named generics inside of the typing system.

use crate::base::kind::BaseTypeKind;

pub mod kind;

pub struct BaseType {
    /// The kind of the base type
    pub kind: BaseTypeKind,

    /// The size specifiers of the type
    pub size_specifiers: Vec<usize>,
}

impl BaseType {
    pub fn new(kind: BaseTypeKind, size_specifiers: Vec<usize>) -> Self {
        Self {
            kind,
            size_specifiers,
        }
    }
}

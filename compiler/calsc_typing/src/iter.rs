//! Calscin typing iterator definitions

use crate::tree::Type;

/// Represents a type that is iterable
pub trait IterableType {
    /// Is the type iterable with the given type as the index type.
    fn is_iterable(&self, ty: Type) -> bool;

    /// Gets the iterator output type.
    fn get_iterator_output_type(&self) -> Type;
}

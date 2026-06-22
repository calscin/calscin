//! The kind of type used.

use calsc_utils::alloc::arena::ArenaHandle;

/// The state of mutation of a type.
/// A false value represents that the type is immutable.
/// A true value represents that the type is mutable.
pub struct MutationState(pub bool);

/// The kind of type. Represents types. Uses the arena allocator to contain inner types
pub enum TypeKind {
    /// Represents a reference.
    ///
    /// # Example
    /// `s.32&` is an immutable reference of type `s.32`
    ///
    /// The handle represents a [`TypeKind`]
    ///
    Reference(MutationState, ArenaHandle),

    /// Represents a pointer.
    ///
    /// # Example
    /// `s.32* mut` is a mutable pointer of type `s.32`
    ///
    /// The handle represents a [`TypeKind`]
    ///
    Pointer(MutationState, ArenaHandle),
}

//! The kind of type used.

use calsc_utils::alloc::arena::ArenaHandle;

/// The state of mutation of a type.
/// A false value represents that the type is immutable.
/// A true value represents that the type is mutable.
#[derive(PartialEq, Eq, Hash)]
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

    /// Represents a compile-time sized array.
    ///
    /// # Example
    /// `s.32[32]` is a 32-sized array of type `s.32`
    ///
    /// The handle represents a [`TypeKind`]
    ///
    Array(usize, ArenaHandle),

    /// A segment represents a continuous segment of memory that has an array-like representation.
    ///
    /// The handle represents a [`TypeKind`]
    Segment(ArenaHandle),
}

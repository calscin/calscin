//! The kind of type used.

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_no_require_type_parameter, build_requires_type_parameter},
};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::types::primitive::PrimitiveType;

pub mod fmt;
pub mod primitive;
pub mod structs;

/// The state of mutation of a type.
/// A false value represents that the type is immutable.
/// A true value represents that the type is mutable.
#[derive(PartialEq, Eq, Hash)]
pub struct MutationState(pub bool);

/// The state of mutation of a type.
/// A value of 0 represents that the size parameter is inactive
/// A value of >= 1 represents the size of the size parameter.
#[derive(PartialEq, Eq, Hash)]
pub struct SizeParameter(pub usize);

/// The kind of type. Represents types. Uses the arena allocator to contain inner types
#[derive(PartialEq)]
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

    /// A primitive type represents a primitive type instance with a size parameter.
    Primitive(PrimitiveType, SizeParameter),
}

impl SizeParameter {
    /// Is the size parameter valid / active.
    pub fn is_active(&self) -> bool {
        self.0 > 0
    }
}

impl TypeKind {
    /// Safely creates a new primitive by checking the need of size parameters.
    ///
    /// # Errors
    /// This function will error if the primitive requires a size specifier and there isn't one and vice-versa.
    ///
    pub fn new_primitive<S: DiagnosticSource>(
        primitive: PrimitiveType,
        param: SizeParameter,
        source: &S,
    ) -> DiagResult<Self> {
        if primitive.requires_size_parameter() != param.is_active() {
            if !primitive.requires_size_parameter() {
                return Err(build_no_require_type_parameter(&primitive, source).into());
            }

            return Err(build_requires_type_parameter(&primitive, source).into());
        }

        return Ok(Self::Primitive(primitive, param));
    }
}

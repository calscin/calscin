//! The context of the type system.

use crate::allocs::TypeKindArena;

/// The typing context. Holds temporary / permanent information (eg: allocators).
/// This context should be passed instead of individual references as it doesn't add additional cost to pass.
pub struct TypeCtx {
    /// The arena allocator used for [`TypeKind`][`crate::types::TypeKind`]
    pub type_kind_arena: TypeKindArena,
}

impl TypeCtx {
    pub fn new() -> Self {
        Self {
            type_kind_arena: TypeKindArena::new(),
        }
    }
}

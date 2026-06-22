//! The context of the type system.

use crate::allocs::{StructContainerArena, TypeKindArena};

/// The typing context. Holds temporary / permanent information (eg: allocators).
/// This context should be passed instead of individual references as it doesn't add additional cost to pass.
pub struct TypeCtx {
    /// The arena allocator used for [`TypeKind`][`crate::types::TypeKind`]
    pub type_kind_arena: TypeKindArena,

    /// The arena allocator used for [`StructContainer`][`crate::types::structs::StructContainer`]
    pub struct_container_arena: StructContainerArena,
}

impl TypeCtx {
    pub fn new() -> Self {
        Self {
            type_kind_arena: TypeKindArena::new(),
            struct_container_arena: StructContainerArena::new(),
        }
    }
}

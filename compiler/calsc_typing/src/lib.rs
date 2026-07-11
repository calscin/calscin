//! The renovated typing system of Calscin. Allows the handling of type parameters and more.
//! This version will properly use diagnostics and arena allocation

use calsc_utils::alloc::arena::ArenaAllocator;

use crate::allocs::{StructContainerArena, TypeKindArena, TypedFunctionArena};

pub mod allocs;
pub mod builders;
pub mod cell;
pub mod ctx;
pub mod funcs;
pub mod hash;
pub mod hints;
pub mod into;
pub mod params;
pub mod prelude;
pub mod traits;
pub mod types;

#[derive(Debug)]
pub struct TypingInterner {
    pub struct_container_arena: StructContainerArena,
    pub type_kind_arena: TypeKindArena,
    pub func_arena: TypedFunctionArena,
}

impl TypingInterner {
    pub fn new() -> Self {
        Self {
            struct_container_arena: ArenaAllocator::new(),
            type_kind_arena: ArenaAllocator::new(),
            func_arena: ArenaAllocator::new(),
        }
    }
}

impl Default for TypingInterner {
    fn default() -> Self {
        TypingInterner::new()
    }
}

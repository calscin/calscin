//! Allocator definitions for typing system

use calsc_utils::alloc::arena::ArenaAllocator;

use crate::types::TypeKind;

pub type TypeKindArena = ArenaAllocator<TypeKind>;

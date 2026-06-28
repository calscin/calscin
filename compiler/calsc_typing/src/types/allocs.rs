//! Allocator definitions for typing system

use calsc_utils::alloc::arena::ArenaAllocator;

use crate::{
    funcs::TypedFunction,
    types::{TypeKind, structs::StructContainer},
};

pub type TypeKindArena = ArenaAllocator<TypeKind>;
pub type StructContainerArena = ArenaAllocator<StructContainer>;
pub type TypedFunctionArena = ArenaAllocator<TypedFunction>;

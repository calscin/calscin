//! Allocator definitions for typing system

use calsc_utils::{alloc::arena::ArenaAllocator, unsafes::UnsafeMut};

use crate::{
    funcs::TypedFunction,
    types::{TypeKind, enums::EnumContainer, structs::StructContainer},
};

pub type TypeKindArena = ArenaAllocator<TypeKind>;
pub type StructContainerArena = ArenaAllocator<StructContainer>;
pub type EnumContainerArena = ArenaAllocator<EnumContainer>;
pub type TypedFunctionArena = ArenaAllocator<TypedFunction>;

thread_local! {
    pub static STRUCT_CONTAINER_ALLOC: UnsafeMut<StructContainerArena> = UnsafeMut::new(StructContainerArena::new());
    pub static ENUM_CONTAINER_ALLOC: UnsafeMut<EnumContainerArena> = UnsafeMut::new(EnumContainerArena::new());
}

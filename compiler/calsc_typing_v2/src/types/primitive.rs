//! Definitions for primitive types. Primitive types are the root of types and represent the actual concrete type.

use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::{ctx::TypeCtx, traits::FieldedType, types::TypeKind};

#[derive(PartialEq, Clone)]
pub enum PrimitiveType {
    /// Represents an integer type with a given signed state.
    Int(bool),

    /// Represents a signed float type.
    Float,

    /// Represents a string type
    Str,

    /// Represents a boolean type
    Boolean,

    /// Represents a struct type
    ///
    /// The handle represents a [`StructContainer`][`crate::types::structs::StructContainer`]
    ///
    Struct(ArenaHandle),

    /// Represents a reference to a function or lambda
    ///
    /// The handle represents a [`TypedFunction`][`crate::funcs::TypedFunction`]
    ///
    Function(ArenaHandle),

    /// Represents a size type
    Size,
}

impl PrimitiveType {
    /// Checks whenther the [`PrimitiveType`] requires a size specifier to be created.
    pub fn requires_size_parameter(&self) -> bool {
        match self {
            Self::Int(_) => true,
            Self::Float => true,

            _ => false,
        }
    }
}

impl FieldedType for PrimitiveType {
    fn has_field(&self, name: &HashedString, ctx: &TypeCtx) -> bool {
        match self {
            Self::Struct(container) => ctx
                .struct_container_arena
                .get(container)
                .fields
                .has_field(name, ctx),

            _ => false,
        }
    }

    unsafe fn get_field(&self, field: &HashedString, ctx: &TypeCtx) -> TypeKind {
        match self {
            Self::Struct(container) => unsafe {
                ctx.struct_container_arena
                    .get(container)
                    .fields
                    .get_field(field, ctx)
            },

            _ => panic!("Type cannot hold field"),
        }
    }
}

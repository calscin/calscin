//! Definitions for primitive types. Primitive types are the root of types and represent the actual concrete type.

use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::{
    allocs::{ENUM_CONTAINER_ALLOC, STRUCT_CONTAINER_ALLOC},
    ctx::TypeCtx,
    params::TypeParameterId,
    traits::FieldedType,
    types::TypeKind,
};

#[cfg_attr(feature = "debug", derive(Debug))]
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

    /// Represents an enum type
    ///
    /// The handle represents an [`EnumContainer`][`crate::types::enums::EnumContainer`]
    ///
    Enum(ArenaHandle),

    /// Represents an enum entry type
    ///
    /// The handle represents an [`EnumContainer`][`crate::types::enums::EnumContainer`]
    /// The hashed string represents the entry name
    ///
    EnumEntry(ArenaHandle, HashedString),

    /// Represents a reference to a function or lambda
    ///
    /// The handle represents a [`TypedFunction`][`crate::funcs::TypedFunction`]
    ///
    Function(ArenaHandle),

    TypeParameter(TypeParameterId),

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

    pub fn is_numeric(&self) -> bool {
        matches!(self, Self::Int(_) | Self::Float | Self::Size)
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float)
    }

    pub fn is_size(&self) -> bool {
        matches!(self, Self::Size)
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_enum_entry(&self) -> bool {
        matches!(self, Self::EnumEntry(_, _))
    }

    pub fn get_signed_state(&self) -> bool {
        match self {
            Self::Int(signed) => *signed,
            _ => false,
        }
    }
}

impl FieldedType for PrimitiveType {
    fn has_field(&self, name: &HashedString, ctx: &TypeCtx) -> bool {
        match self {
            Self::Struct(container) => STRUCT_CONTAINER_ALLOC
                .with(|f| f.borrow().get(container).fields.has_field(name, ctx)),

            Self::EnumEntry(container, name) => ENUM_CONTAINER_ALLOC.with(|f| {
                f.borrow().get(container).entries[&name]
                    .fields
                    .has_field(name, ctx)
            }),

            _ => false,
        }
    }

    fn get_fields(&self, ctx: &TypeCtx) -> Vec<HashedString> {
        match self {
            Self::Struct(container) => {
                STRUCT_CONTAINER_ALLOC.with(|f| f.borrow().get(container).fields.get_fields(ctx))
            }

            Self::EnumEntry(container, name) => ENUM_CONTAINER_ALLOC.with(|f| {
                f.borrow().get(&container).entries[&name]
                    .fields
                    .get_fields(ctx)
            }),

            _ => vec![],
        }
    }

    fn get_field_index(&self, field: &HashedString, ctx: &TypeCtx) -> usize {
        match self {
            Self::Struct(container) => STRUCT_CONTAINER_ALLOC
                .with(|f| f.borrow().get(container).fields.get_field_index(field, ctx)),

            Self::EnumEntry(container, name) => ENUM_CONTAINER_ALLOC.with(|f| {
                f.borrow().get(container).entries[&name]
                    .fields
                    .get_field_index(field, ctx)
            }),

            _ => panic!("Type cannot hold field"),
        }
    }

    unsafe fn get_field(&self, field: &HashedString, ctx: &TypeCtx) -> TypeKind {
        match self {
            Self::Struct(container) => unsafe {
                STRUCT_CONTAINER_ALLOC
                    .with(|f| f.borrow().get(container).fields.get_field(field, ctx))
            },

            Self::EnumEntry(container, name) => ENUM_CONTAINER_ALLOC.with(|f| unsafe {
                f.borrow().get(container).entries[&name]
                    .fields
                    .get_field(field, ctx)
            }),

            _ => panic!("Type cannot hold field"),
        }
    }
}

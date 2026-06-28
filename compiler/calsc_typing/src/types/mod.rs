//! The kind of type used.

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{
        build_expected_type_parameters_error, build_no_require_type_parameter,
        build_requires_type_parameter,
    },
};
use calsc_utils::{alloc::arena::ArenaHandle, display_with_to_string, hash::HashedString};

use crate::{
    ctx::TypeCtx,
    traits::{FieldedType, TypeParameteredType},
    types::primitive::PrimitiveType,
};

pub mod fmt;
pub mod primitive;
pub mod structs;

/// The state of mutation of a type.
/// A false value represents that the type is immutable.
/// A true value represents that the type is mutable.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MutationState(pub bool);

/// The state of mutation of a type.
/// A value of 0 represents that the size parameter is inactive
/// A value of >= 1 represents the size of the size parameter.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct SizeParameter(pub usize);

/// A primitive held inside of [`TypeKind`]
#[derive(Clone, Debug, PartialEq)]
pub struct HeldPrimitive {
    pub ty: PrimitiveType,
    pub size: SizeParameter,
    pub type_parameters: HashMap<HashedString, ArenaHandle>,
}

/// The kind of type. Represents types. Uses the arena allocator to contain inner types
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
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
    Primitive(HeldPrimitive),

    /// Represents a void type. A void type basically means that the value has no type
    Void,
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
        type_parameters: Vec<TypeKind>,
        ctx: &mut TypeCtx,
        source: &S,
    ) -> DiagResult<Self> {
        if primitive.requires_size_parameter() != param.is_active() {
            if !primitive.requires_size_parameter() {
                return Err(build_no_require_type_parameter(
                    &display_with_to_string(&primitive, ctx),
                    source,
                )
                .into());
            }

            return Err(build_requires_type_parameter(
                &display_with_to_string(&primitive, ctx),
                source,
            )
            .into());
        }

        let ty_params = primitive.get_type_params(ctx);

        if type_parameters.len() != ty_params.len() {
            return Err(build_expected_type_parameters_error(
                &ty_params.len(),
                &type_parameters.len(),
                source,
            )
            .into());
        }

        let mut type_params = HashMap::new();

        for (ind, param) in type_parameters.iter().enumerate() {
            type_params.insert(
                ty_params[ind].clone(),
                ctx.type_kind_arena.append(param.clone()),
            );
        }

        return Ok(Self::Primitive(HeldPrimitive {
            ty: primitive,
            size: param,
            type_parameters: type_params,
        }));
    }

    pub fn get_inner<'a>(&self, ctx: &'a TypeCtx) -> &'a TypeKind {
        match self {
            Self::Array(_, inner) => ctx.type_kind_arena.get(inner),
            Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner),
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner),

            _ => panic!(
                "Type {} doesn't contain any inner type",
                display_with_to_string(self, ctx)
            ),
        }
    }

    /// Checks whenther the type is compatible with mutation operations.
    /// This mostly will be used for references and pointers
    pub fn is_mutation_compatible(&self) -> bool {
        match self {
            Self::Pointer(mutable, _) => mutable.0,
            Self::Reference(mutable, _) => mutable.0,

            _ => true,
        }
    }

    pub fn is_directly_numeric(&self) -> bool {
        match self {
            Self::Primitive(primitive) => primitive.ty.is_numeric(),
            _ => false,
        }
    }

    pub fn as_primitive(&self) -> HeldPrimitive {
        match self {
            Self::Primitive(primitive) => primitive.clone(),

            #[cfg(feature = "debug")]
            _ => panic!("Direct type of {:#?} is not primitive!", self),

            #[cfg(not(feature = "debug"))]
            _ => panic!("Direct type of type is not primitive!"),
        }
    }

    pub fn is_static(&self, ctx: &TypeCtx) -> bool {
        match self {
            Self::Primitive(_) => true,
            Self::Reference(_, _) => false,
            Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner).is_static(ctx),
            Self::Array(_, inner) => ctx.type_kind_arena.get(inner).is_static(ctx),
            Self::Segment(inner) => ctx.type_kind_arena.get(inner).is_static(ctx),
            Self::Void => false,
        }
    }

    pub fn is_safe_for_struct_storage(&self, ctx: &TypeCtx) -> bool {
        self.is_static(ctx)
    }

    pub fn is_directly_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }

    pub fn is_directly_primitive(&self) -> bool {
        matches!(self, Self::Primitive(_))
    }
}

impl FieldedType for TypeKind {
    fn has_field(&self, name: &HashedString, ctx: &TypeCtx) -> bool {
        match self {
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).has_field(name, ctx),
            Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner).has_field(name, ctx),
            Self::Primitive(primitive) => primitive.ty.has_field(name, ctx),

            _ => false,
        }
    }

    fn get_fields(&self, ctx: &TypeCtx) -> Vec<HashedString> {
        match self {
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).get_fields(ctx),
            Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner).get_fields(ctx),
            Self::Primitive(primitive) => primitive.ty.get_fields(ctx),

            _ => vec![],
        }
    }

    fn get_field_index(&self, field: &HashedString, ctx: &TypeCtx) -> usize {
        match self {
            Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).get_field_index(field, ctx),
            Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner).get_field_index(field, ctx),
            Self::Primitive(primitive) => primitive.ty.get_field_index(field, ctx),

            _ => panic!("Type cannot hold fields!"),
        }
    }

    unsafe fn get_field(&self, field: &HashedString, ctx: &TypeCtx) -> TypeKind {
        unsafe {
            match self {
                Self::Reference(_, inner) => ctx.type_kind_arena.get(inner).get_field(field, ctx),
                Self::Pointer(_, inner) => ctx.type_kind_arena.get(inner).get_field(field, ctx),
                Self::Primitive(primitive) => primitive.ty.get_field(field, ctx),

                _ => panic!("Type cannot hold fields!"),
            }
        }
    }
}

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
    TypingInterner,
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
        interner: &mut TypingInterner,
        source: &S,
    ) -> DiagResult<Self> {
        if primitive.requires_size_parameter() != param.is_active() {
            if !primitive.requires_size_parameter() {
                return Err(build_no_require_type_parameter(
                    &display_with_to_string(&primitive, interner),
                    source,
                )
                .into());
            }

            return Err(build_requires_type_parameter(
                &display_with_to_string(&primitive, interner),
                source,
            )
            .into());
        }

        let ty_params = primitive.get_type_params(interner);

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
                interner.type_kind_arena.append(param.clone()),
            );
        }

        return Ok(Self::Primitive(HeldPrimitive {
            ty: primitive,
            size: param,
            type_parameters: type_params,
        }));
    }

    pub fn get_inner<'a>(&self, interner: &'a TypingInterner) -> &'a TypeKind {
        match self {
            Self::Array(_, inner) => interner.type_kind_arena.get(inner),
            Self::Pointer(_, inner) => interner.type_kind_arena.get(inner),
            Self::Reference(_, inner) => interner.type_kind_arena.get(inner),

            _ => panic!(
                "Type {} doesn't contain any inner type",
                display_with_to_string(self, interner)
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

    pub fn is_static(&self, interner: &TypingInterner) -> bool {
        match self {
            Self::Primitive(_) => true,
            Self::Reference(_, _) => false,
            Self::Pointer(_, inner) => interner.type_kind_arena.get(inner).is_static(interner),
            Self::Array(_, inner) => interner.type_kind_arena.get(inner).is_static(interner),
            Self::Segment(inner) => interner.type_kind_arena.get(inner).is_static(interner),
            Self::Void => false,
        }
    }

    pub fn is_safe_for_struct_storage(&self, interner: &TypingInterner) -> bool {
        self.is_static(interner)
    }

    pub fn is_directly_array(&self) -> bool {
        matches!(self, Self::Array(_, _))
    }

    pub fn is_directly_primitive(&self) -> bool {
        matches!(self, Self::Primitive(_))
    }

    pub fn lower_type_parameter_type(&self, ty: TypeKind, interner: &TypingInterner) -> TypeKind {
        match self {
            Self::Primitive(primitive) => primitive.lower_type_parameter_type(ty, interner),
            Self::Array(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .lower_type_parameter_type(ty, interner),

            Self::Pointer(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .lower_type_parameter_type(ty, interner),

            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .lower_type_parameter_type(ty, interner),

            Self::Segment(inner) => interner
                .type_kind_arena
                .get(inner)
                .lower_type_parameter_type(ty, interner),

            Self::Void => ty,
        }
    }
}

impl FieldedType for TypeKind {
    fn has_field(&self, name: &HashedString, ctx: &TypeCtx, interner: &TypingInterner) -> bool {
        match self {
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .has_field(name, ctx, interner),
            Self::Pointer(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .has_field(name, ctx, interner),
            Self::Primitive(primitive) => primitive.ty.has_field(name, ctx, interner),

            _ => false,
        }
    }

    fn get_fields(&self, ctx: &TypeCtx, interner: &TypingInterner) -> Vec<HashedString> {
        match self {
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_fields(ctx, interner),
            Self::Pointer(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_fields(ctx, interner),
            Self::Primitive(primitive) => primitive.ty.get_fields(ctx, interner),

            _ => vec![],
        }
    }

    fn get_field_index(
        &self,
        field: &HashedString,
        ctx: &TypeCtx,
        interner: &TypingInterner,
    ) -> usize {
        match self {
            Self::Reference(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_field_index(field, ctx, interner),
            Self::Pointer(_, inner) => interner
                .type_kind_arena
                .get(inner)
                .get_field_index(field, ctx, interner),
            Self::Primitive(primitive) => primitive.ty.get_field_index(field, ctx, interner),

            _ => panic!("Type cannot hold fields!"),
        }
    }

    unsafe fn get_field(
        &self,
        field: &HashedString,
        ctx: &TypeCtx,
        interner: &TypingInterner,
    ) -> TypeKind {
        unsafe {
            let ty = match self {
                Self::Reference(_, inner) => interner
                    .type_kind_arena
                    .get(inner)
                    .get_field(field, ctx, interner),
                Self::Pointer(_, inner) => interner
                    .type_kind_arena
                    .get(inner)
                    .get_field(field, ctx, interner),
                Self::Primitive(primitive) => primitive.ty.get_field(field, ctx, interner),

                _ => panic!("Type cannot hold fields!"),
            };

            self.lower_type_parameter_type(ty, interner)
        }
    }
}

impl HeldPrimitive {
    pub(crate) fn get_type_parameter_type_value(
        &self,
        name: HashedString,
        interner: &TypingInterner,
    ) -> Option<TypeKind> {
        if !self.type_parameters.contains_key(&name) {
            None
        } else {
            Some(
                interner
                    .type_kind_arena
                    .get(&self.type_parameters[&name])
                    .clone(),
            )
        }
    }

    pub fn lower_type_parameter_type(&self, ty: TypeKind, interner: &TypingInterner) -> TypeKind {
        if let TypeKind::Primitive(primitive) = &ty {
            if let PrimitiveType::TypeParameter(param) = &primitive.ty {
                let lowered = self.get_type_parameter_type_value(param.1.clone(), interner);

                if lowered.is_some() {
                    return lowered.unwrap();
                }
            }
        }

        ty
    }
}

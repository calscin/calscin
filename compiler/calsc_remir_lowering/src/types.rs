use calsc_diagnostics::DiagResult;

use calsc_typing::{
    ctx::TypeCtx,
    traits::FieldedType,
    types::{HeldPrimitive, TypeKind, primitive::PrimitiveType},
};
use remir::values::ValueType;

#[allow(unsafe_code)]
pub fn lower_type_base(ty: HeldPrimitive, ctx: &TypeCtx) -> DiagResult<ValueType> {
    match ty.0 {
        PrimitiveType::Boolean => Ok(ValueType::Int(false, 1)),
        PrimitiveType::Int(signed) => Ok(ValueType::Int(signed, ty.1.0)),
        PrimitiveType::Float => Ok(ValueType::Float(ty.1.0)),
        PrimitiveType::Str => Ok(ValueType::new_any_pointer()),
        PrimitiveType::Struct(container) => {
            let mut type_fields = vec![];
            let container = ctx.struct_container_arena.get(&container);

            for field in container.fields.get_fields(ctx) {
                let field_ty = unsafe { container.fields.get_field(&field, ctx) }; // This is safe since get_fields return the list of fields

                type_fields.push(Box::new(lower_type(field_ty, ctx)?))
            }

            Ok(ValueType::Struct(type_fields))
        }

        PrimitiveType::Size => Ok(ValueType::new_int(false, usize::BITS as usize)),
        PrimitiveType::Function(_) => Ok(ValueType::new_any_pointer()),
    }
}

pub fn lower_type(ty: TypeKind, ctx: &TypeCtx) -> DiagResult<ValueType> {
    match ty {
        TypeKind::Primitive(primitive, size) => {
            lower_type_base(HeldPrimitive(primitive, size), ctx)
        }

        TypeKind::Reference(_, inner) => {
            let inner = ctx.type_kind_arena.get(&inner).clone();

            return Ok(ValueType::Pointer(Box::new(lower_type(inner, ctx)?)));
        }

        TypeKind::Pointer(_, inner) => {
            let inner = ctx.type_kind_arena.get(&inner).clone();

            return Ok(ValueType::Pointer(Box::new(lower_type(inner, ctx)?)));
        }

        TypeKind::Array(size, inner) => {
            let inner = ctx.type_kind_arena.get(&inner).clone();

            Ok(ValueType::Array(
                Box::new(lower_type(inner, ctx)?),
                Some(size),
            ))
        }

        TypeKind::Segment(inner) => {
            let inner = ctx.type_kind_arena.get(&inner).clone();

            Ok(ValueType::Array(Box::new(lower_type(inner, ctx)?), None))
        }

        TypeKind::Void => Ok(ValueType::Void),
    }
}

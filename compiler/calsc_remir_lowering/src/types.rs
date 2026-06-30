use std::collections::HashMap;

use calsc_diagnostics::DiagResult;

use calsc_typing::{
    allocs::{ENUM_CONTAINER_ALLOC, STRUCT_CONTAINER_ALLOC},
    ctx::TypeCtx,
    traits::FieldedType,
    types::{HeldPrimitive, SizeParameter, TypeKind, primitive::PrimitiveType},
};
use remir::values::ValueType;

#[allow(unsafe_code)]
pub fn lower_type_base(ty: HeldPrimitive, ctx: &TypeCtx) -> DiagResult<ValueType> {
    match ty.ty.clone() {
        PrimitiveType::Boolean => Ok(ValueType::Int(false, 1)),
        PrimitiveType::Int(signed) => Ok(ValueType::Int(signed, ty.size.0)),
        PrimitiveType::Float => Ok(ValueType::Float(ty.size.0)),
        PrimitiveType::Str => Ok(ValueType::new_any_pointer()),
        PrimitiveType::Struct(container) => {
            let mut type_fields = vec![];
            let container = STRUCT_CONTAINER_ALLOC.with(|f| f.borrow().get(&container).clone());

            for field in container.fields.get_fields(ctx) {
                let field_ty = unsafe { container.fields.get_field(&field, ctx) }; // This is safe since get_fields return the list of fields
                let field_ty = ty.lower_type_parameter_type(field_ty, ctx);

                type_fields.push(Box::new(lower_type(field_ty, ctx)?))
            }

            Ok(ValueType::Struct(type_fields))
        }

        PrimitiveType::Enum(container_ref) => {
            let mut max_size = 0;
            let mut marker_type = TypeKind::Void;

            ENUM_CONTAINER_ALLOC.with(|f| {
                let container = f.borrow().get(&container_ref);

                marker_type = container.get_marker_type();

                for (entry, _) in &container.entries {
                    let sz = lower_type_base(
                        HeldPrimitive {
                            ty: PrimitiveType::EnumEntry(container_ref.clone(), entry.clone()),
                            size: SizeParameter(0),
                            type_parameters: ty.type_parameters.clone(),
                        },
                        ctx,
                    )?
                    .get_size();

                    if sz > max_size {
                        max_size = sz;
                    }
                }

                Ok::<(), ()>(())
            })?;

            Ok(ValueType::Struct(vec![
                Box::new(lower_type(marker_type, ctx)?),
                Box::new(ValueType::Int(false, max_size)),
            ]))
        }

        PrimitiveType::EnumEntry(container_ref, name) => ENUM_CONTAINER_ALLOC.with(|f| {
            let container = f.borrow().get(&container_ref);
            let entry = &container.entries[&name];

            let marker_type = container.get_marker_type();

            let mut fields = vec![];

            fields.push(Box::new(lower_type(marker_type, ctx)?));

            for field in entry.fields.get_fields(ctx) {
                let field_ty = unsafe { entry.fields.get_field(&field, ctx) }; // This is safe since get_fields return the list of fields
                let field_ty = ty.lower_type_parameter_type(field_ty, ctx);

                fields.push(Box::new(lower_type(field_ty, ctx)?))
            }

            Ok(ValueType::Struct(fields))
        }),

        PrimitiveType::Size => Ok(ValueType::new_int(false, usize::BITS as usize)),
        PrimitiveType::Function(_) => Ok(ValueType::new_any_pointer()),
        PrimitiveType::TypeParameter(param) => {
            lower_type(ctx.type_params.get_resolved(&param.1), ctx)
        }
    }
}

pub fn lower_type(ty: TypeKind, ctx: &TypeCtx) -> DiagResult<ValueType> {
    match ty {
        TypeKind::Primitive(primitive) => lower_type_base(primitive, ctx),

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

use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::{BUILD_CACHE, HIRContext};
use calsc_modules::lazy::LazyLoadedType;
use calsc_typing::types::{
    HeldPrimitive, MutationState, SizeParameter, TypeKind, primitive::PrimitiveType,
};

pub fn lower_module_path_type<S: DiagnosticSource>(
    ty: LazyLoadedType,
    origin: &S,
    hir_ctx: &mut HIRContext,
) -> DiagResult<TypeKind> {
    match ty {
        LazyLoadedType::TypeParameter { id: _, name } => {
            let res = hir_ctx.type_ctx.type_params.get_type_param(&name, origin)?;

            Ok(TypeKind::Primitive(HeldPrimitive {
                ty: PrimitiveType::TypeParameter(res),
                size: SizeParameter(0),
            }))
        }

        LazyLoadedType::Base {
            module_path,
            element_name,
            size_specifiers,
        } => {
            let mut new_path = module_path.clone();
            new_path.append_single_bit(element_name);

            let primitive =
                BUILD_CACHE.with_borrow(|cache| cache.type_storage.map[&new_path].clone());

            Ok(TypeKind::Primitive(HeldPrimitive {
                ty: primitive,
                size: SizeParameter(size_specifiers),
            }))
        }

        LazyLoadedType::Array { size, inner } => {
            let inner = lower_module_path_type(*inner, origin, hir_ctx)?;
            let inner = hir_ctx.type_ctx.type_kind_arena.append(inner);

            if size.is_some() {
                Ok(TypeKind::Array(size.unwrap(), inner))
            } else {
                Ok(TypeKind::Segment(inner))
            }
        }

        LazyLoadedType::Pointer { mutable, inner } => {
            let inner = lower_module_path_type(*inner, origin, hir_ctx)?;
            let inner = hir_ctx.type_ctx.type_kind_arena.append(inner);

            Ok(TypeKind::Pointer(MutationState(mutable), inner))
        }

        LazyLoadedType::Reference { mutable, inner } => {
            let inner = lower_module_path_type(*inner, origin, hir_ctx)?;
            let inner = hir_ctx.type_ctx.type_kind_arena.append(inner);

            Ok(TypeKind::Reference(MutationState(mutable), inner))
        }

        LazyLoadedType::Void => Ok(TypeKind::Void),
    }
}

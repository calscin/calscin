use std::collections::HashMap;

use calsc_ast::{path::ElementPath, types::ASTType};
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_tree_build::utils::resolve_path;
use calsc_typing::types::{
    HeldPrimitive, MutationState, SizeParameter, TypeKind, primitive::PrimitiveType,
};

use crate::ctx::TreeLowCtx;

pub fn lower_ast_type<S: DiagnosticSource>(
    ty: &ASTType,
    ctx: &mut TreeLowCtx,
    source: &S,
) -> DiagResult<TypeKind> {
    match ty {
        ASTType::Array(size, inner) => {
            let inner = lower_ast_type(&*inner, ctx, source)?;
            let inner = ctx.type_ctx.interner.type_kind_arena.append(inner);

            if size.is_none() {
                Ok(TypeKind::Segment(inner))
            } else {
                Ok(TypeKind::Array(size.unwrap(), inner))
            }
        }

        ASTType::Pointer(mutable, inner) => {
            let inner = lower_ast_type(&*inner, ctx, source)?;
            let inner = ctx.type_ctx.interner.type_kind_arena.append(inner);

            Ok(TypeKind::Pointer(MutationState(*mutable), inner))
        }

        ASTType::Reference(mutable, inner) => {
            let inner = lower_ast_type(&*inner, ctx, source)?;
            let inner = ctx.type_ctx.interner.type_kind_arena.append(inner);

            Ok(TypeKind::Reference(MutationState(*mutable), inner))
        }

        ASTType::Generic(name, size_spec, type_params) => {
            lower_ast_type_generic(name, size_spec, type_params, ctx, source)
        }

        ASTType::Void => Ok(TypeKind::Void),
    }
}

pub fn lower_ast_type_generic<S: DiagnosticSource>(
    name: &ElementPath,
    size_spec: &Option<usize>,
    type_params: &Vec<Box<ASTType>>,
    ctx: &mut TreeLowCtx,
    source: &S,
) -> DiagResult<TypeKind> {
    // Handle type parameters
    if name.members.len() == 1 {
        if ctx
            .type_ctx
            .type_params
            .has_type_parameter(&name.members[0])
        {
            let param = ctx
                .type_ctx
                .type_params
                .get_type_param(&name.members[0], source)?;

            return Ok(TypeKind::Primitive(HeldPrimitive {
                ty: PrimitiveType::TypeParameter(param),
                size: SizeParameter(0),
                type_parameters: HashMap::new(),
            }));
        }
    }

    let path = resolve_path(name.clone(), &ctx.build_ctx, source)?;
    let mut lowered_type_params: Vec<TypeKind> = vec![];

    for type_param in type_params {
        lowered_type_params.push(lower_ast_type(&*type_param, ctx, source)?);
    }

    let mut size_specifier = SizeParameter(0);

    if size_spec.is_some() {
        size_specifier = SizeParameter(size_spec.unwrap());
    }

    let primitive = ctx.get_entry(&path, source)?.as_type(source)?;

    Ok(TypeKind::new_primitive(
        primitive.0.clone(),
        size_specifier,
        lowered_type_params,
        &mut ctx.type_ctx,
        source,
    )?)
}

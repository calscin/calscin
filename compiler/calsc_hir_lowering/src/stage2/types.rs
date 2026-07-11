use std::collections::HashMap;

use calsc_ast::{path::ElementPath, types::ASTType};
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::HIRContext;
use calsc_tree_build::utils::resolve_path;
use calsc_typing::types::{
    HeldPrimitive, MutationState, SizeParameter, TypeKind, primitive::PrimitiveType,
};

pub fn lower_ast_type<'ctx, 'session: 'ctx, S: DiagnosticSource>(
    ty: &ASTType,
    origin: &S,
    hir_ctx: &'ctx mut HIRContext<'session>,
) -> DiagResult<TypeKind> {
    match ty {
        ASTType::Array(size, inner) => {
            let inner = lower_ast_type(&*inner, origin, hir_ctx)?;
            let inner = hir_ctx.session.type_interner.type_kind_arena.append(inner);

            if size.is_none() {
                Ok(TypeKind::Segment(inner))
            } else {
                Ok(TypeKind::Array(size.unwrap(), inner))
            }
        }

        ASTType::Pointer(mutable, inner) => {
            let inner = lower_ast_type(&*inner, origin, hir_ctx)?;
            let inner = hir_ctx.session.type_interner.type_kind_arena.append(inner);

            Ok(TypeKind::Pointer(MutationState(*mutable), inner))
        }

        ASTType::Reference(mutable, inner) => {
            let inner = lower_ast_type(&*inner, origin, hir_ctx)?;
            let inner = hir_ctx.session.type_interner.type_kind_arena.append(inner);

            Ok(TypeKind::Reference(MutationState(*mutable), inner))
        }

        ASTType::Generic(name, size_spec, type_params) => {
            lower_ast_type_generic_interner(name, size_spec, type_params, hir_ctx, origin)
        }

        ASTType::Void => Ok(TypeKind::Void),
    }
}

pub fn lower_ast_type_generic_interner<'ctx, 'session: 'ctx, S: DiagnosticSource>(
    name: &ElementPath,
    size_spec: &Option<usize>,
    type_params: &Vec<Box<ASTType>>,
    ctx: &'ctx mut HIRContext<'session>,
    source: &S,
) -> DiagResult<TypeKind> {
    // Handle type parameters
    if name.members.len() == 1 {
        if ctx
            .session
            .get_tree_lowered()
            .type_ctx
            .type_params
            .has_type_parameter(&name.members[0])
        {
            let param = ctx
                .session
                .get_tree_lowered()
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

    let path = resolve_path(
        name.clone(),
        &ctx.session.get_tree_lowered().build_ctx,
        source,
    )?;
    let mut lowered_type_params: Vec<TypeKind> = vec![];

    for type_param in type_params {
        lowered_type_params.push(lower_ast_type(&*type_param, source, ctx)?);
    }

    let mut size_specifier = SizeParameter(0);

    if size_spec.is_some() {
        size_specifier = SizeParameter(size_spec.unwrap());
    }

    let primitive = ctx
        .session
        .get_tree_lowered()
        .get_entry(&path, source)?
        .as_type(source)?;

    Ok(TypeKind::new_primitive(
        primitive.0.clone(),
        size_specifier,
        lowered_type_params,
        &mut ctx.session.type_interner,
        source,
    )?)
}

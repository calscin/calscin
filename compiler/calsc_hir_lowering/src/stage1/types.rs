use calsc_ast::{
    nodes::{ASTNode, ASTNodeKind},
    path::ElementPath,
    types::ASTType,
};
use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{
        build_compile_time_size, build_expected_simple_type, build_internal_hir_node_leaked,
        build_type_not_static,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};

use calsc_typing::types::{
    MutationState, SizeParameter, TypeKind,
    primitive::PrimitiveType,
    structs::{NamedField, StructContainer},
};
use calsc_utils::display_with_to_string;

use crate::{convert_visibility, stage2::key::lower_ast_key};

pub fn lower_ast_struct_declaration(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
        fields,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        let key = GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

        let mut struct_container =
            StructContainer::new(name.clone(), file_ctx.current_module.clone());

        for field in fields {
            let ty = lower_ast_type(field.0, &node, file_ctx, ctx)?; // We can clone base_type to pass it to lower_ast_type since the base_type here wont be modified by lower_ast_type

            if !ty.is_static(&ctx.type_ctx) {
                return Err(build_type_not_static(
                    &display_with_to_string(&ty, &ctx.type_ctx),
                    &node,
                )
                .into());
            }

            struct_container
                .fields
                .append_named(NamedField(field.1, ty), &node)?;
        }

        let struct_container = ctx.type_ctx.struct_container_arena.append(struct_container);

        ctx.scope.append(
            key,
            GlobalContextValue::Type(PrimitiveType::Struct(struct_container)),
            visibility,
            &node,
        )?;

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_simple_ast_type<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<PrimitiveType> {
    if let ASTType::Generic(generic, size_param) = ty.clone() {
        if size_param.is_some() {
            return Err(build_expected_simple_type(origin).into());
        }

        let ty = lower_ast_generic_base(generic, 0, origin, file_ctx, ctx)?;

        if let TypeKind::Primitive(primitive, size) = ty {
            if size.is_active() {
                return Ok(primitive);
            }
        }
    }

    return Err(build_expected_simple_type(origin).into());
}

pub fn lower_ast_type<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<TypeKind> {
    lower_ast_type_complex(ty, origin, false, file_ctx, ctx)
}

pub fn lower_ast_type_complex<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    allow_compile_time_uncertain_types: bool,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<TypeKind> {
    match ty.clone() {
        ASTType::Array(size, b) => {
            if size.is_none() && !allow_compile_time_uncertain_types {
                return Err(build_compile_time_size(&ty, origin).into());
            }

            let inner = lower_ast_type_complex(
                *b,
                origin,
                allow_compile_time_uncertain_types,
                file_ctx,
                ctx,
            )?;

            let inner = ctx.type_ctx.type_kind_arena.append(inner);

            if size.is_some() {
                Ok(TypeKind::Array(size.unwrap(), inner))
            } else {
                Ok(TypeKind::Segment(inner))
            }
        }

        ASTType::Reference(mutable, inner) => {
            let inner = lower_ast_type_complex(
                *inner,
                origin,
                allow_compile_time_uncertain_types,
                file_ctx,
                ctx,
            )?;

            let inner = ctx.type_ctx.type_kind_arena.append(inner);

            Ok(TypeKind::Reference(MutationState(mutable), inner))
        }

        ASTType::Pointer(mutable, inner) => {
            let inner = lower_ast_type_complex(
                *inner,
                origin,
                allow_compile_time_uncertain_types,
                file_ctx,
                ctx,
            )?;

            let inner = ctx.type_ctx.type_kind_arena.append(inner);

            Ok(TypeKind::Pointer(MutationState(mutable), inner))
        }

        ASTType::Generic(generic, size_param) => {
            let mut size_specifiers = 0;

            if size_param.is_some() {
                size_specifiers = size_param.unwrap();
            }

            let ty = lower_ast_generic_base(generic, size_specifiers, origin, file_ctx, ctx)?;

            Ok(ty)
        }

        ASTType::Void => Ok(TypeKind::Void),
    }
}

pub fn lower_ast_generic_base<K: DiagnosticSource>(
    name: ElementPath,
    size_specifier: usize,
    origin: &K,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<TypeKind> {
    // If the name is relative and of a length of one, we check first if it's a type parameter
    if name.relative && name.members.len() == 1 {
        if ctx
            .type_ctx
            .type_params
            .has_type_parameter(&name.members[0])
        {
            let param = ctx
                .type_ctx
                .type_params
                .get_type_param(&name.members[0], origin)?;

            return Ok(TypeKind::Primitive(
                PrimitiveType::TypeParameter(param),
                SizeParameter(0),
            ));
        }
    }

    let key = lower_ast_key(name, origin, true, file_ctx, ctx)?;

    let ty = ctx
        .scope
        .get_entry(key, &file_ctx.current_module, origin)?
        .craft_type(origin, &ctx.type_ctx, SizeParameter(size_specifier))?;

    Ok(ty)
}

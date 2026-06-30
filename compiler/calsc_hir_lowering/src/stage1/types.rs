use std::collections::HashMap;

use calsc_ast::{
    nodes::{ASTNode, ASTNodeKind},
    path::ElementPath,
    types::ASTType,
};
use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{
        build_compile_time_size, build_enum_entry_already_present, build_expected_simple_type,
        build_internal_hir_node_leaked, build_type_not_static,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};

use calsc_typing::{
    allocs::{ENUM_CONTAINER_ALLOC, STRUCT_CONTAINER_ALLOC},
    types::{
        HeldPrimitive, MutationState, SizeParameter, TypeKind,
        enums::{EnumContainer, EnumEntryContainer},
        primitive::PrimitiveType,
        structs::{NamedField, StructContainer},
    },
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
        type_parameters,
    } = node.kind.clone()
    {
        let group = ctx.type_ctx.type_params.start_param_group();

        let mut struct_container =
            StructContainer::new(name.clone(), file_ctx.current_module.clone());

        for type_parameter in type_parameters {
            ctx.type_ctx
                .type_params
                .append_type_param(type_parameter.clone(), &node)?;

            struct_container.type_parameters.push(type_parameter);
        }

        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        let key = GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

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

        let struct_container =
            STRUCT_CONTAINER_ALLOC.with(|f| f.borrow_mut().append(struct_container));

        ctx.scope.append(
            key,
            GlobalContextValue::Type(PrimitiveType::Struct(struct_container)),
            visibility,
            &node,
        )?;

        ctx.type_ctx.type_params.end_group(group);

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_enum_declaration(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagPossible {
    if let ASTNodeKind::EnumDeclaration {
        name,
        entries,
        visibility,
        type_parameters,
    } = node.kind.clone()
    {
        let group = ctx.type_ctx.type_params.start_param_group();

        let mut enum_container = EnumContainer::new(name.clone(), file_ctx.current_module.clone());

        for type_parameter in type_parameters {
            ctx.type_ctx
                .type_params
                .append_type_param(type_parameter.clone(), &node)?;

            enum_container.type_parameters.push(type_parameter);
        }

        let enum_container = ENUM_CONTAINER_ALLOC.with(|f| f.borrow_mut().append(enum_container));

        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        let key = GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

        for (entry_name, fields) in entries {
            let mut enum_entry = EnumEntryContainer::new(entry_name.clone(), enum_container);

            for field in fields {
                let ty = lower_ast_type(field.0, &node, file_ctx, ctx)?;

                if !ty.is_static(&ctx.type_ctx) {
                    return Err(build_type_not_static(
                        &display_with_to_string(&ty, &ctx.type_ctx),
                        &node,
                    )
                    .into());
                }

                enum_entry
                    .fields
                    .append_named(NamedField(field.1, ty), &node)?;
            }

            println!("Test: {:#?}", enum_entry.fields);

            let has_entry = ENUM_CONTAINER_ALLOC.with(|f| {
                f.borrow()
                    .get(&enum_container)
                    .entries
                    .contains_key(&entry_name)
            });

            if has_entry {
                return Err(build_enum_entry_already_present(&entry_name, &node).into());
            }

            ENUM_CONTAINER_ALLOC.with(|f| {
                f.borrow_mut()
                    .get_mut(&enum_container)
                    .append_entry(entry_name.clone(), enum_entry);
            });

            // Append to HIR context
            let entry_key = GlobalContextKey::new(entry_name.clone()).associated_type(key.clone());

            ctx.scope.append(
                entry_key,
                GlobalContextValue::Type(PrimitiveType::EnumEntry(
                    enum_container.clone(),
                    entry_name,
                )),
                visibility.clone(),
                &node,
            )?;
        }

        ctx.scope.append(
            key,
            GlobalContextValue::Type(PrimitiveType::Enum(enum_container)),
            visibility,
            &node,
        )?;

        ctx.type_ctx.type_params.end_group(group);
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
    if let ASTType::Generic(generic, size_param, type_parameters) = ty.clone() {
        if size_param.is_some() || !type_parameters.is_empty() {
            return Err(build_expected_simple_type(origin).into());
        }

        let ty = lower_ast_generic_base(generic, 0, type_parameters, origin, file_ctx, ctx)?;

        if let TypeKind::Primitive(primitive) = ty {
            if primitive.size.is_active() {
                return Ok(primitive.ty);
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

        ASTType::Generic(generic, size_param, type_parameters) => {
            let mut size_specifiers = 0;

            if size_param.is_some() {
                size_specifiers = size_param.unwrap();
            }

            let ty = lower_ast_generic_base(
                generic,
                size_specifiers,
                type_parameters,
                origin,
                file_ctx,
                ctx,
            )?;

            Ok(ty)
        }

        ASTType::Void => Ok(TypeKind::Void),
    }
}

pub fn lower_ast_generic_base<K: DiagnosticSource>(
    name: ElementPath,
    size_specifier: usize,
    type_parameters: Vec<Box<ASTType>>,
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

            return Ok(TypeKind::Primitive(HeldPrimitive {
                ty: PrimitiveType::TypeParameter(param),
                size: SizeParameter(0),
                type_parameters: HashMap::new(),
            }));
        }
    }

    let key = lower_ast_key(name, origin, true, file_ctx, ctx)?;

    let mut lowered_type_parameters = vec![];

    for type_parameter in type_parameters {
        lowered_type_parameters.push(lower_ast_type_complex(
            *type_parameter,
            origin,
            false,
            file_ctx,
            ctx,
        )?)
    }

    let ty = ctx
        .scope
        .get_entry(key, &file_ctx.current_module, origin)?
        .craft_type(
            origin,
            &mut ctx.type_ctx,
            SizeParameter(size_specifier),
            lowered_type_parameters,
        )?;

    Ok(ty)
}

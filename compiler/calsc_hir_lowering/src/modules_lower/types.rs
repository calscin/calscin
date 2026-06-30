use std::collections::HashMap;

use calsc_ast::{
    nodes::{ASTNode, ASTNodeKind},
    types::ASTType,
};
use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{
        build_expected_entry_type, build_internal_hir_node_leaked, build_type_not_static,
        build_unexpected_type_alias_additional_parameters,
    },
};
use calsc_hir::{BUILD_CACHE, file::HIRFileContext};
use calsc_modules::{
    lazy::LazyLoadedTypeLike,
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};

use calsc_typing::{
    allocs::STRUCT_CONTAINER_ALLOC,
    ctx::TypeCtx,
    traits::TypeParameteredType,
    types::{
        HeldPrimitive, MutationState, SizeParameter, TypeKind,
        primitive::PrimitiveType,
        structs::{NamedField, StructContainer},
    },
};
use calsc_utils::{display_with_to_string, hash::HashedCounter};

use crate::stage0::fill::types::lower_stage0_key;

pub fn lower_type_from_tree(
    path: ModulePath,
    tree: &ModuleTree,
    type_ctx: &mut TypeCtx,
) -> DiagPossible {
    let already_built = BUILD_CACHE.with_borrow(|cache| cache.type_storage.map.contains_key(&path));

    if already_built {
        return Ok(()); // No need to do anything else if its already built.
    }

    let source = BUILD_CACHE.with_borrow(|cache| cache.nodes_to_entries[&path][0].clone());

    let r = tree.traverse_to(path.clone(), &source)?;

    if let ModuleTreeEntry::FilledType(ty) = r {
        println!("Type to lower: {:#?}", ty);

        let mut dependencies = HashedCounter::new();

        ty.get_dependencies(tree, &mut dependencies, &source)?;

        // Lower each dependency first so that the type can be safely resolved
        for dependency in &dependencies.map {
            lower_type_from_tree(dependency.0.clone(), tree, type_ctx)?;
        }

        // We first build a HIR file context with the current module path to allow for same-module resolution
        let module_path = path.everything_but_last();

        let hir_file_ctx = HIRFileContext {
            current_module: module_path,
            lazy_imports: vec![],
        };

        // We then lower each part of the type by lowering each related AST node.
        let related_nodes = BUILD_CACHE.with_borrow(|cache| cache.nodes_to_entries[&path].clone());

        for node in related_nodes {
            lower_type_node(path.clone(), tree, node, &hir_file_ctx, type_ctx)?;
        }
    } else {
        return Err(build_expected_entry_type(&"type".to_string(), &path, &source).into());
    }

    Ok(())
}

pub fn lower_type<S: DiagnosticSource>(
    ty: ASTType,
    tree: &ModuleTree,
    hir_file_ctx: &HIRFileContext,
    type_ctx: &mut TypeCtx,
    source: &S,
) -> DiagResult<TypeKind> {
    match ty {
        ASTType::Array(size, inner) => {
            let inner = lower_type(*inner, tree, hir_file_ctx, type_ctx, source)?;
            let inner = type_ctx.type_kind_arena.append(inner);

            if size.is_some() {
                Ok(TypeKind::Array(size.unwrap(), inner))
            } else {
                Ok(TypeKind::Segment(inner))
            }
        }

        ASTType::Reference(mutable, inner) => {
            let inner = lower_type(*inner, tree, hir_file_ctx, type_ctx, source)?;
            let inner = type_ctx.type_kind_arena.append(inner);

            Ok(TypeKind::Reference(MutationState(mutable), inner))
        }

        ASTType::Pointer(mutable, inner) => {
            let inner = lower_type(*inner, tree, hir_file_ctx, type_ctx, source)?;
            let inner = type_ctx.type_kind_arena.append(inner);

            Ok(TypeKind::Pointer(MutationState(mutable), inner))
        }

        ASTType::Generic(name, size_specs, type_parameters) => {
            let mut size_specifier = 0;

            if size_specs.is_some() {
                size_specifier = size_specs.unwrap();
            }

            let (mut path, element_name) = lower_stage0_key(name, hir_file_ctx, tree);
            path.append_single_bit(element_name.clone());

            if type_ctx.type_params.has_type_parameter(&element_name) {
                if size_specs.is_some() || !type_parameters.is_empty() {
                    return Err(build_unexpected_type_alias_additional_parameters(source).into());
                }

                let type_param = type_ctx.type_params.get_type_param(&element_name, source)?;

                return Ok(TypeKind::Primitive(HeldPrimitive {
                    ty: PrimitiveType::TypeParameter(type_param),
                    size: SizeParameter(0),
                    type_parameters: HashMap::new(),
                }));
            }

            let raw_type = BUILD_CACHE.with_borrow(|state| state.type_storage.map[&path].clone());

            let ty_type_parameters = raw_type.get_type_params(type_ctx);
            let mut lowered_type_params = HashMap::new();

            for (ind, type_parameter) in type_parameters.iter().enumerate() {
                let ty = lower_type(
                    *type_parameter.clone(),
                    tree,
                    hir_file_ctx,
                    type_ctx,
                    source,
                )?;

                let ty = type_ctx.type_kind_arena.append(ty);

                lowered_type_params.insert(ty_type_parameters[ind].clone(), ty);
            }

            Ok(TypeKind::Primitive(HeldPrimitive {
                ty: raw_type,
                size: SizeParameter(size_specifier),
                type_parameters: lowered_type_params,
            }))
        }

        ASTType::Void => Ok(TypeKind::Void),
    }
}

pub fn lower_type_node(
    path: ModulePath,
    tree: &ModuleTree,
    node: ASTNode,
    hir_file_ctx: &HIRFileContext,
    type_ctx: &mut TypeCtx,
) -> DiagPossible {
    match node.kind {
        ASTNodeKind::StructDeclaration { .. } => {
            lower_type_struct_decl(path, tree, node, hir_file_ctx, type_ctx)
        }

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    }
}

pub fn lower_type_struct_decl(
    path: ModulePath,
    tree: &ModuleTree,
    node: ASTNode,
    hir_file_ctx: &HIRFileContext,
    type_ctx: &mut TypeCtx,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
        fields,
        visibility: _,
        type_parameters,
    } = node.kind.clone()
    {
        let group = type_ctx.type_params.start_param_group();

        let mut container = StructContainer::new(name, hir_file_ctx.current_module.clone());

        for type_parameter in type_parameters {
            container.type_parameters.push(type_parameter.clone());

            type_ctx
                .type_params
                .append_type_param(type_parameter, &node)?;
        }

        for field in fields {
            let ty = lower_type(field.0, tree, hir_file_ctx, type_ctx, &node)?;

            if !ty.is_safe_for_struct_storage(type_ctx) {
                return Err(
                    build_type_not_static(&display_with_to_string(&ty, type_ctx), &node).into(),
                );
            }

            container
                .fields
                .append_named(NamedField(field.1, ty), &node)?;
        }

        let container = STRUCT_CONTAINER_ALLOC.with(|f| f.borrow_mut().append(container));

        BUILD_CACHE.with_borrow_mut(|cache| {
            cache
                .type_storage
                .map
                .insert(path, PrimitiveType::Struct(container))
        });

        type_ctx.type_params.end_group(group);

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

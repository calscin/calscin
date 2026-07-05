use std::path::PathBuf;

use calsc_ast::{
    imports::ImportKind,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{
        build_ambiguous_import_name, build_cannot_find_element_no_closest,
        build_internal_hir_node_leaked,
    },
};
use calsc_modules::{path::PackageLessModulePath, treev2::imports::ImportFilter};

use crate::{
    ctx::TreeBuildingCtx,
    utils::{matches_any_import, resolve_import_path},
    walk_second_pass::types::{get_semantic_deps_inner, get_typing_deps_inner},
};

pub fn walk_second_pass_node(
    node: &ASTNode,
    _path: &PathBuf,
    ctx: &mut TreeBuildingCtx,
) -> DiagPossible {
    match &node.kind {
        ASTNodeKind::StructDeclaration { .. } => walk_second_pass_struct(node, ctx),
        ASTNodeKind::FunctionDeclaration { .. } => Ok(()),
        ASTNodeKind::ExternFunctionDeclaration { .. } => Ok(()),
        ASTNodeKind::Module { .. } => todo!(),

        _ => return Err(build_internal_hir_node_leaked(&node, node).into()),
    }
}

pub fn walk_second_pass_struct(node: &ASTNode, ctx: &mut TreeBuildingCtx) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name: _,
        fields,
        visibility: _,
        type_parameters,
    } = node.kind.clone()
    {
        let mut semantic_deps = vec![];
        let mut typing_deps = vec![];

        for field in fields {
            get_semantic_deps_inner(&field.0, ctx, &type_parameters, &mut semantic_deps, node)?;
            get_typing_deps_inner(&field.0, ctx, &mut typing_deps, &type_parameters, node)?;
        }

        let entry = ctx
            .tree
            .get_entry_mut(&ctx.current_path, &mut ctx.arena, node)?;

        for semantic in semantic_deps {
            entry.semantic_dependencies.insert(semantic);
        }

        for typing in typing_deps {
            entry.typing_dependencies.insert(typing);
        }

        Ok(())
    } else {
        unreachable!()
    }
}

pub fn walk_second_pass_import(
    node: &ASTNode,
    _file_path: &PathBuf,
    ctx: &mut TreeBuildingCtx,
) -> DiagPossible {
    if let ASTNodeKind::ImportStatement { path, kind } = node.kind.clone() {
        let module_handle = ctx
            .tree
            .get_entry_handle(&ctx.current_path, &ctx.arena, node)?
            .clone();

        match kind {
            ImportKind::Module => {
                let path = resolve_import_path(path, ctx.current_path.clone(), ctx);

                if !ctx.tree.has_entry(&path, &ctx.arena) {
                    return Err(build_cannot_find_element_no_closest(&path, node).into());
                }

                let module = ctx.arena.get_mut(&module_handle).kind.as_module_mut(node)?;

                if matches_any_import(module, &PackageLessModulePath(vec![path.last()])) {
                    return Err(build_ambiguous_import_name(&path.last(), node).into());
                }

                module.imports.push(ImportFilter::new(
                    vec![path.last()],
                    PackageLessModulePath::from(path.into()).0,
                ))
            }

            ImportKind::Items(items) => {
                let module_immutable = ctx.arena.get(&module_handle).kind.as_module(node)?.clone();

                for item in items {
                    let mut path = path.clone();
                    path.members.push(item);

                    let path = resolve_import_path(path, ctx.current_path.clone(), ctx);

                    if !ctx.tree.has_entry(&path, &ctx.arena) {
                        return Err(build_cannot_find_element_no_closest(&path, node).into());
                    }

                    if matches_any_import(
                        &module_immutable,
                        &PackageLessModulePath(vec![path.last()]),
                    ) {
                        return Err(build_ambiguous_import_name(&path, node).into());
                    }

                    let module = ctx.arena.get_mut(&module_handle).kind.as_module_mut(node)?;

                    module.imports.push(ImportFilter::new(
                        vec![path.last()],
                        PackageLessModulePath::from(path.into()).0,
                    ))
                }
            }

            ImportKind::Whole => {
                let module_immutable = ctx.arena.get(&module_handle).kind.as_module(node)?.clone();

                let path = resolve_import_path(path, ctx.current_path.clone(), ctx);

                let whole_module = ctx
                    .tree
                    .get_entry(&path, &ctx.arena, node)?
                    .kind
                    .as_module(node)?
                    .clone();

                for (name, _) in &whole_module.children {
                    let mut path = path.clone();
                    path.append_single_bit(name.clone());

                    if !ctx.tree.has_entry(&path, &ctx.arena) {
                        return Err(build_cannot_find_element_no_closest(&path, node).into());
                    }

                    if matches_any_import(
                        &module_immutable,
                        &PackageLessModulePath(vec![name.clone()]),
                    ) {
                        return Err(build_ambiguous_import_name(&path, node).into());
                    }

                    let module = ctx.arena.get_mut(&module_handle).kind.as_module_mut(node)?;

                    module.imports.push(ImportFilter::new(
                        vec![name.clone()],
                        PackageLessModulePath::from(path.into()).0,
                    ))
                }
            }
        }

        Ok(())
    } else {
        unreachable!()
    }
}

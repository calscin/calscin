use std::path::PathBuf;

use calsc_ast::{
    imports::ImportKind,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{
        build_ambiguous_import_name, build_expected_entry_type, build_internal_hir_node_leaked,
    },
};
use calsc_modules::{
    path::PackageLessModulePath,
    treev2::{ModuleTree, imports::ImportFilter, module::TreeModule},
};

use crate::{
    ctx::TreeBuildingCtx,
    utils::{matches_any_import, resolve_import_path},
};

pub fn walk_second_pass_node(
    node: &ASTNode,
    _path: &PathBuf,
    _ctx: &mut TreeBuildingCtx,
) -> DiagPossible {
    match &node.kind {
        ASTNodeKind::StructDeclaration { .. } => todo!(),
        ASTNodeKind::FunctionDeclaration { .. } => Ok(()),
        ASTNodeKind::ExternFunctionDeclaration { .. } => Ok(()),
        ASTNodeKind::Module { .. } => todo!(),

        _ => return Err(build_internal_hir_node_leaked(&node, node).into()),
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

            _ => todo!(),
        }

        Ok(())
    } else {
        unreachable!()
    }
}

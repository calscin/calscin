use std::path::PathBuf;

use calsc_ast::{
    imports::ImportKind,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{build_ambiguous_import_name, build_internal_hir_node_leaked},
};
use calsc_modules::{path::PackageLessModulePath, treev2::imports::ImportFilter};

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
        let module = ctx
            .tree
            .get_entry_mut(&ctx.current_path, &mut ctx.arena, node)?
            .kind
            .as_module_mut(node)?;

        match kind {
            ImportKind::Module => {
                let path = resolve_import_path(path, ctx.current_path.clone());

                if matches_any_import(module, &path.clone().into()) {
                    return Err(build_ambiguous_import_name(&path.last(), node).into());
                }

                module.imports.push(ImportFilter::new(
                    vec![path.last()],
                    PackageLessModulePath::from(path.into()).0,
                ))
            }

            ImportKind::Items(items) => {
                for item in items {
                    let mut path = path.clone();
                    path.members.push(item);

                    let path = resolve_import_path(path, ctx.current_path.clone());

                    if matches_any_import(module, &path.clone().into()) {
                        return Err(build_ambiguous_import_name(&path, node).into());
                    }

                    module.imports.push(ImportFilter::new(
                        vec![path.last()],
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

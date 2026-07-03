//! The file walking / processing stage.
//! Basically processes the AST context in order to build the module tree

use std::path::PathBuf;

use calsc_ast::{ASTContext, nodes::ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{ctx::TreeBuildingCtx, walk::modules::walk_through_module};

pub mod modules;

pub fn walk_in_file(path: &PathBuf, ast: &ASTContext, ctx: &mut TreeBuildingCtx) -> DiagPossible {
    for node in &ast.tree {
        walk_through_node(node, ast, ctx)?;
    }

    Ok(())
}

pub fn walk_through_node(
    node: &ArenaHandle,
    ast: &ASTContext,
    ctx: &mut TreeBuildingCtx,
) -> DiagPossible {
    let node_ref = ast.nodes.get(node);

    match node_ref.kind {
        ASTNodeKind::Module { .. } => walk_through_module(node, ast, ctx),
        ASTNodeKind::StructDeclaration { .. } => Ok(()),
        ASTNodeKind::ExternFunctionDeclaration { .. } => Ok(()),
        ASTNodeKind::FunctionDeclaration { .. } => Ok(()),

        _ => return Err(build_internal_hir_node_leaked(node_ref, node_ref).into()),
    }
}

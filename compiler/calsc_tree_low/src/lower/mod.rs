use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_modules::path::ModulePath;

use crate::{
    ctx::TreeLowCtx,
    lower::{
        funcs::{lower_ast_extern_function_declaration, lower_ast_function_declaration},
        structs::lower_ast_struct_declaration,
    },
};

pub mod funcs;
pub mod structs;

pub fn lower_ast_node(node: ASTNode, ctx: &mut TreeLowCtx, path: ModulePath) -> DiagPossible {
    match node.kind {
        ASTNodeKind::StructDeclaration { .. } => lower_ast_struct_declaration(node, ctx, path),
        ASTNodeKind::FunctionDeclaration { .. } => lower_ast_function_declaration(node, ctx, path),
        ASTNodeKind::ExternFunctionDeclaration { .. } => {
            lower_ast_extern_function_declaration(node, ctx, path)
        }

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    }
}

pub fn lower_ast_entry(path: ModulePath, ctx: &mut TreeLowCtx) -> DiagPossible {
    if !ctx.build_ctx.related_nodes.contains_key(&path) {
        return Ok(());
    }

    let (_, nodes) = ctx.build_ctx.related_nodes[&path].clone();

    // TODO: add priority sorting
    for node in nodes {̣̣
        lower_ast_node(node, ctx, path.clone())?;
    }

    Ok(())
}

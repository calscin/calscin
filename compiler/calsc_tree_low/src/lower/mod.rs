use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_modules::path::ModulePath;

use crate::{
    ctx::TreeLowCtx,
    get_dependencies_of_entry,
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

    // We lower the dependencies first

    let deps = get_dependencies_of_entry(ctx, &path, &ctx.build_ctx.related_nodes[&path].1[0])?;

    for dep in deps {
        lower_ast_entry(dep, ctx)?;
    }

    let (_, nodes) = ctx.build_ctx.related_nodes[&path].clone();

    // TODO: add priority sorting
    for node in nodes {
        lower_ast_node(node, ctx, path.clone())?;
    }

    Ok(())
}

pub fn lower_everything(ctx: &mut TreeLowCtx) -> DiagPossible {
    for path in ctx
        .build_ctx
        .related_nodes
        .keys()
        .map(ModulePath::clone)
        .collect::<Vec<_>>()
    {
        lower_ast_entry(path, ctx)?;
    }

    Ok(())
}

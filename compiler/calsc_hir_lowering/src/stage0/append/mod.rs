//! The append pass of stage 0.
//! The goal of this pass is to append "empty" entries inside of the module tree for types and functions.

use std::path::PathBuf;

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::file::HIRFileContext;
use calsc_modules::tree::{ModuleTree, entry::ModuleTreeEntry};

use crate::stage0::append::{
    funcs::{
        lower_stage_0_append_pass_extern_function_declaration,
        lower_stage_0_append_pass_function_declaration,
    },
    types::lower_stage_0_append_pass_struct_declaration,
};

pub mod funcs;
pub mod types;

pub fn lower_stage_0_append_pass(
    ast_ctx: &ASTContext,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
    file_path: PathBuf,
) -> DiagPossible {
    // Append file path to the module
    {
        let mut_ref = tree.traverse_mutably_to(
            file_ctx.current_module.clone(),
            ast_ctx.nodes.get(&ast_ctx.tree[0]),
        )?;

        if let ModuleTreeEntry::Module(module) = mut_ref {
            module.path = Some(file_path);
        }
    }

    for node in &ast_ctx.tree {
        lower_stage_0_append_pass_node(ast_ctx.nodes.get(node).clone(), &ast_ctx, file_ctx, tree)?;
    }

    Ok(())
}

fn lower_stage_0_append_pass_node(
    node: ASTNode,
    ast_ctx: &ASTContext,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    match node.kind {
        ASTNodeKind::StructDeclaration { .. } => {
            lower_stage_0_append_pass_struct_declaration(node, file_ctx, tree)
        }

        ASTNodeKind::FunctionDeclaration { .. } => {
            lower_stage_0_append_pass_function_declaration(node, file_ctx, tree)
        }

        ASTNodeKind::ExternFunctionDeclaration { .. } => {
            lower_stage_0_append_pass_extern_function_declaration(node, file_ctx, tree)
        }

        ASTNodeKind::Module { .. } => {
            lower_stage_0_append_pass_module(node, ast_ctx, file_ctx, tree)
        }

        ASTNodeKind::StructDeclBlock { .. } => Ok(()),

        ASTNodeKind::ImportStatement { .. } => Ok(()),

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    }
}

fn lower_stage_0_append_pass_module(
    node: ASTNode,
    ast_ctx: &ASTContext,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    if let ASTNodeKind::Module {
        name,
        is_bodied: _,
        body,
    } = node.kind.clone()
    {
        file_ctx.advance_module(name);

        for node in body {
            let node = ast_ctx.nodes.get(&node).clone();

            lower_stage_0_append_pass_node(node, ast_ctx, file_ctx, tree)?;
        }

        file_ctx.deadvance_module();

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

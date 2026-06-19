//! The fill pass of the stage 0
//! The goal of this pass is to fill the "empty" entries inside of the module tree for types and functions.
//! The fill phase also manages tracking of which modules are imported or not
//!
//! We use an append pass first in order to allow for all

use std::path::PathBuf;

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{build_already_in_scope, build_internal_hir_node_leaked},
};
use calsc_hir::file::HIRFileContext;
use calsc_modules::tree::{ModuleTree, entry::ModuleTreeEntry};

use crate::stage0::fill::{
    func::{lower_ast_extern_func_decl_stage_zero, lower_ast_function_decl_stage_zero},
    lower_types::lower_ast_type_struct_declaration,
};

pub mod func;
pub mod key;
pub mod lower_types;
pub mod prelude;
pub mod types;

pub fn lower_stage_0_fill_pass(
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
        let node = ast_ctx.nodes.get(node).clone();

        lower_stage_0_node(node, &ast_ctx, file_ctx, tree)?;
    }

    Ok(())
}

pub fn lower_stage_0_node(
    node: ASTNode,
    ast_ctx: &ASTContext,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    match node.kind {
        ASTNodeKind::FunctionDeclaration { .. } => {
            lower_ast_function_decl_stage_zero(node, file_ctx, tree)
        }

        ASTNodeKind::ExternFunctionDeclaration { .. } => {
            lower_ast_extern_func_decl_stage_zero(node, file_ctx, tree)
        }

        ASTNodeKind::StructDeclaration { .. } => {
            lower_ast_type_struct_declaration(node, file_ctx, tree)
        }

        ASTNodeKind::StructDeclBlock { .. } => Ok(()),

        ASTNodeKind::Module { .. } => lower_ast_stage_0_module(node, ast_ctx, file_ctx, tree),

        ASTNodeKind::ImportStatement { .. } => Ok(()),

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    }
}

pub fn lower_ast_stage_0_module(
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
        file_ctx.advance_module(name.clone());

        {
            let path_to_mutate = file_ctx.current_module.clone();

            let mut_ref = tree.traverse_mutably_to(path_to_mutate.clone(), &node)?;

            if let ModuleTreeEntry::Module(module) = mut_ref {
                module.imported = true;
            } else {
                return Err(build_already_in_scope(&path_to_mutate, &node).into());
            }
        }

        for body_node in body {
            let body_node = ast_ctx.nodes.get(&body_node).clone();

            lower_stage_0_node(body_node, ast_ctx, file_ctx, tree)?;
        }

        file_ctx.deadvance_module();

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

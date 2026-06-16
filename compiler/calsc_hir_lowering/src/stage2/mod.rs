//! Declarations for the second stage of the HIR lowering. The stage 2 has one responsibility:
//! - Parse functions bodies
//!
//! The stage 2 should propagate the function body implementations

use std::path::PathBuf;

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, file::HIRFileContext};
use calsc_modules::{path::ModulePath, tree::ModuleTree};

use crate::{
    modules::module_tree_append_file,
    stage2::{funcs::lower_ast_function_decl, structs::lower_ast_struct_decl},
};

pub mod control;
pub mod funcs;
pub mod key;
pub mod structs;
pub mod values;
pub mod vars;

pub fn lower_hir_stage_2(
    ast_context: ASTContext,
    ctx: &mut HIRContext,
    file_ctx: &mut HIRFileContext,
) -> DiagPossible {
    let mut tree = ModuleTree::new();

    module_tree_append_file(
        PathBuf::from("meow.cal"),
        ModulePath::new("test_package".into(), vec!["meow".into()]),
        &mut tree,
        &ASTNode::clone(ast_context.nodes.get(&ast_context.tree[0])),
    )?;

    println!("{:#?}", tree);

    for node in &ast_context.tree {
        lower_hir_stage_2_node(
            ASTNode::clone(ast_context.nodes.get(node)),
            file_ctx,
            ctx,
            &ast_context,
        )?;
    }

    Ok(())
}

pub fn lower_hir_stage_2_node(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_context: &ASTContext,
) -> DiagPossible {
    match &node.kind {
        ASTNodeKind::FunctionDeclaration { .. } => {
            let _ =
                lower_ast_function_decl(ASTNode::clone(&node), None, file_ctx, ctx, ast_context)?;
        }

        ASTNodeKind::ExternFunctionDeclaration { .. } => return Ok(()),
        ASTNodeKind::StructDeclaration { .. } => return Ok(()),

        ASTNodeKind::StructDeclBlock { .. } => {
            lower_ast_struct_decl(ASTNode::clone(&node), file_ctx, ctx, ast_context)?
        }

        ASTNodeKind::Module { .. } => lower_hir_stage_2_module(node, file_ctx, ctx, ast_context)?,

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    }

    Ok(())
}

pub fn lower_hir_stage_2_module(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_context: &ASTContext,
) -> DiagPossible {
    if let ASTNodeKind::Module { name, body } = node.kind.clone() {
        file_ctx.advance_module(name);

        for element in body {
            lower_hir_stage_2_node(
                ASTNode::clone(ast_context.nodes.get(&element)),
                file_ctx,
                ctx,
                ast_context,
            )?;
        }

        file_ctx.deadvance_module();

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

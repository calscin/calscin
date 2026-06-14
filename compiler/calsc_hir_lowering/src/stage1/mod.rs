//! Declarations for the first stage of the HIR lowering. The stage 1 has a couple of responsibilities:
//! - Add types to the global scope
//! - Add stage 1 functions to the global scope
//! - Add extern function to the global scope
//!
//! The stage 1 should only create the local context and append the arguments inside

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{
    HIR_CONTEXT,
    file::{self, HIRFileContext},
    prelude::apply_prelude,
};

use crate::stage1::{
    funcs::{lower_ast_extern_function, lower_ast_function_decl_first_stage},
    types::{lower_ast_decl_block, lower_ast_struct_declaration},
};

pub mod funcs;
pub mod types;

pub fn lower_hir_stage_1_node(node: ASTNode, file_ctx: &mut HIRFileContext) -> DiagPossible {
    match node.kind {
        ASTNodeKind::FunctionDeclaration { .. } => {
            lower_ast_function_decl_first_stage(ASTNode::clone(&node), None, file_ctx)?;
        }

        ASTNodeKind::ExternFunctionDeclaration { .. } => {
            lower_ast_extern_function(ASTNode::clone(&node))?
        }
        ASTNodeKind::StructDeclaration { .. } => {
            lower_ast_struct_declaration(ASTNode::clone(&node))?
        }

        ASTNodeKind::StructDeclBlock { .. } => {
            lower_ast_decl_block(ASTNode::clone(&node), file_ctx)?
        }

        ASTNodeKind::Module { .. } => lower_hir_stage_1_module(node, file_ctx)?,

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    };

    Ok(())
}

pub fn lower_hir_stage_1(ast_context: ASTContext) -> DiagPossible {
    let mut first = false;
    let mut file_ctx = HIRFileContext::new();

    for node in &ast_context.tree {
        if !first {
            first = true;

            HIR_CONTEXT.with(|f| apply_prelude(&mut f.borrow_mut().scope, &**node))?;
        }

        lower_hir_stage_1_node(ASTNode::clone(&node), &mut file_ctx)?;
    }

    Ok(())
}

pub fn lower_hir_stage_1_module(node: ASTNode, file_ctx: &mut HIRFileContext) -> DiagPossible {
    if let ASTNodeKind::Module { name, body } = node.kind.clone() {
        file_ctx.advance_module(name);

        for element in body {
            lower_hir_stage_1_node(ASTNode::clone(&element), file_ctx)?;
        }

        file_ctx.deadvance_module();

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

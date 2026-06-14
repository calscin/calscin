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
use calsc_hir::{HIR_CONTEXT, file::HIRFileContext, prelude::apply_prelude};

use crate::stage1::{
    funcs::{lower_ast_extern_function, lower_ast_function_decl_first_stage},
    types::{lower_ast_decl_block, lower_ast_struct_declaration},
};

pub mod funcs;
pub mod types;

pub fn lower_hir_stage_1(ast_context: ASTContext) -> DiagPossible {
    let mut first = false;
    let mut file_ctx = HIRFileContext::new();

    for iter in ast_context.tree_order {
        let node = ast_context.tree[&iter].clone();

        if !first {
            first = true;

            HIR_CONTEXT.with(|f| apply_prelude(&mut f.borrow_mut().scope, &*node))?;
        }

        match node.kind {
            ASTNodeKind::FunctionDeclaration { .. } => {
                lower_ast_function_decl_first_stage(ASTNode::clone(&node), None, &mut file_ctx)?;
            }

            ASTNodeKind::ExternFunctionDeclaration { .. } => {
                lower_ast_extern_function(ASTNode::clone(&node))?
            }
            ASTNodeKind::StructDeclaration { .. } => {
                lower_ast_struct_declaration(ASTNode::clone(&node))?
            }

            _ => return Err(build_internal_hir_node_leaked(&node, &*node).into()),
        };
    }

    for iter in ast_context.additional_tree {
        match &iter.kind {
            ASTNodeKind::StructDeclBlock { .. } => {
                lower_ast_decl_block(ASTNode::clone(&iter), &mut file_ctx)?
            }

            _ => return Err(build_internal_hir_node_leaked(&iter, &*iter).into()),
        }
    }

    Ok(())
}

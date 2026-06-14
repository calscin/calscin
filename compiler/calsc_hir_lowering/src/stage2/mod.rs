//! Declarations for the second stage of the HIR lowering. The stage 2 has one responsibility:
//! - Parse functions bodies
//!
//! The stage 2 should propagate the function body implementations

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::file::HIRFileContext;

use crate::stage2::{funcs::lower_ast_function_decl, structs::lower_ast_struct_decl};

pub mod control;
pub mod funcs;
pub mod key;
pub mod structs;
pub mod values;
pub mod vars;

pub fn lower_hir_stage_2(ast_context: ASTContext) -> DiagPossible {
    let mut file_ctx = HIRFileContext::new();

    for iter in ast_context.tree_order {
        let node = ast_context.tree[&iter].clone();

        match &node.kind {
            ASTNodeKind::FunctionDeclaration { .. } => {
                let _ = lower_ast_function_decl(ASTNode::clone(&node), None, &mut file_ctx)?;
            }

            ASTNodeKind::ExternFunctionDeclaration { .. } => continue,
            ASTNodeKind::StructDeclaration { .. } => continue,

            _ => return Err(build_internal_hir_node_leaked(&node, &*node).into()),
        }
    }

    for iter in ast_context.additional_tree {
        match &iter.kind {
            ASTNodeKind::StructDeclBlock { .. } => {
                lower_ast_struct_decl(ASTNode::clone(&iter), &mut file_ctx)?
            }

            _ => return Err(build_internal_hir_node_leaked(&iter, &*iter).into()),
        }
    }

    Ok(())
}

//! Declarations for the second stage of the HIR lowering. The stage 2 has one responsibility:
//! - Parse functions bodies
//!
//! The stage 2 should propagate the function body implementations

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::DiagPossible;

use crate::stage2::{funcs::lower_ast_function_decl, structs::lower_ast_struct_decl};

pub mod control;
pub mod funcs;
pub mod structs;
pub mod values;
pub mod vars;

pub fn lower_hir_stage_2(ast_context: ASTContext) -> DiagPossible {
    for iter in ast_context.tree_order {
        let node = ast_context.tree[&iter].clone();

        match &node.kind {
            ASTNodeKind::FunctionDeclaration { .. } => {
                let _ = lower_ast_function_decl(ASTNode::clone(&node), None)?;
            }

            ASTNodeKind::ExternFunctionDeclaration { .. } => continue,
            ASTNodeKind::StructDeclaration { .. } => continue,

            kind => panic!("Unexpected {:#?}", kind),
        }
    }

    for iter in ast_context.additional_tree {
        match &iter.kind {
            ASTNodeKind::StructDeclBlock { .. } => lower_ast_struct_decl(ASTNode::clone(&iter))?,

            kind => panic!("Unexpected {:#?}", kind),
        }
    }

    Ok(())
}

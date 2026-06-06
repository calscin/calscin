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
use calsc_diagnostics::DiagPossible;
use calsc_hir::{HIR_CONTEXT, prelude::apply_prelude};

use crate::stage1::{
    funcs::{lower_ast_extern_function, lower_ast_function_decl_first_stage},
    types::{lower_ast_decl_block, lower_ast_struct_declaration},
};

pub mod funcs;
pub mod types;

pub fn lower_hir_stage_1(ast_context: ASTContext) -> DiagPossible {
    let mut first = false;

    for iter in ast_context.tree_order {
        let node = ast_context.tree[&iter].clone();

        if !first {
            first = true;

            HIR_CONTEXT.with_borrow_mut(|f| apply_prelude(&mut f.scope, &*node))?;
        }

        match node.kind {
            ASTNodeKind::FunctionDeclaration { .. } => {
                lower_ast_function_decl_first_stage(ASTNode::clone(&node), None)?;
            }

            ASTNodeKind::ExternFunctionDeclaration { .. } => {
                lower_ast_extern_function(ASTNode::clone(&node))?
            }
            ASTNodeKind::StructDeclaration { .. } => {
                lower_ast_struct_declaration(ASTNode::clone(&node))?
            }

            _ => panic!(),
        };
    }

    for iter in ast_context.additional_tree {
        match &iter.kind {
            ASTNodeKind::StructDeclBlock { .. } => lower_ast_decl_block(ASTNode::clone(&iter))?,

            _ => panic!(),
        }
    }

    Ok(())
}

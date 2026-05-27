//! Lowering for values

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagResult;
use calsc_hir::{localctx::LocalContext, refs::HIRArenaReference};

use crate::stage2::values::{
    booleans::lower_hir_inverse_condition,
    lits::lower_ast_literal,
    ptrs::{lower_ast_pointer_dereference, lower_ast_pointer_reference},
};

pub mod booleans;
pub mod lits;
pub mod ptrs;

/// Lowers an AST value into an HIR value
pub fn lower_ast_value(
    node: ASTNode,
    local_ctx: Option<&LocalContext>,
) -> DiagResult<HIRArenaReference> {
    match node.kind {
        ASTNodeKind::IntLiteral(_)
        | ASTNodeKind::FloatLiteral(_)
        | ASTNodeKind::CharLiteral(_)
        | ASTNodeKind::StringLiteral(_)
        | ASTNodeKind::BooleanLiteral(_) => lower_ast_literal(node),

        ASTNodeKind::InverseCondition(_) => lower_hir_inverse_condition(node, local_ctx),

        ASTNodeKind::PointerReference(_) => lower_ast_pointer_reference(node, local_ctx),
        ASTNodeKind::PointerDereference(_) => lower_ast_pointer_dereference(node, local_ctx),

        _ => unsafe { unreachable_unchecked() },
    }
}

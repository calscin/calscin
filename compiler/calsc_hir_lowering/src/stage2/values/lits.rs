//! Lowering for literals

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagResult;
use calsc_hir::{
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

/// Lowers an AST literal into an HIR literal
pub fn lower_ast_literal(node: ASTNode) -> DiagResult<HIRArenaReference> {
    let kind = match &node.kind {
        ASTNodeKind::IntLiteral(val) => HIRNodeKind::IntLiteral(*val, 128, *val < 0),
        ASTNodeKind::FloatLiteral(val) => HIRNodeKind::FloatLiteral(*val, 128, *val < 0.0),
        ASTNodeKind::StringLiteral(val) => HIRNodeKind::StringLiteral(val.clone()),
        ASTNodeKind::CharLiteral(val) => HIRNodeKind::CharLiteral(*val),
        ASTNodeKind::BooleanLiteral(val) => HIRNodeKind::BooleanLiteral(*val),

        _ => unsafe { unreachable_unchecked() },
    };

    let node = HIRNode::new(kind, node.start.clone(), node.end.clone());

    Ok(node.push())
}

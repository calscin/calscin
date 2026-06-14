//! Lowering for literals

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
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

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    };

    let node = HIRNode::new(kind, node.start.clone(), node.end.clone());

    Ok(node.push())
}

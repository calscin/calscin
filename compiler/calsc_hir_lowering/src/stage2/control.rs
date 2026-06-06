use calsc_ast::{
    ifs::IfStatementBranch,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::DiagResult;
use calsc_hir::{globalctx::key::GlobalContextKey, refs::HIRArenaReference};

use crate::stage2::{funcs::lower_ast_body, values::lower_ast_value};

pub fn lower_ast_if_statement_branch(
    branch: IfStatementBranch,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<calsc_hir::ifs::IfStatementBranch> {
    match branch {
        IfStatementBranch::If { condition, body } => {
            let condition = lower_ast_value(ASTNode::clone(&condition), local_ctx.clone())?;
			let body = lower_ast_body(nodes, local_ctx)
        }
    }
}

pub fn lower_ast_if_statement(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::IfStatement { branches } = node.kind.clone() {
        let mut hir_branches = vec![];
    }
}

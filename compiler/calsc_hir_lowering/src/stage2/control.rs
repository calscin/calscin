use std::hint::unreachable_unchecked;

use calsc_ast::{
    ifs::IfStatementBranch,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::{
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

use crate::stage2::{funcs::lower_ast_body, values::lower_ast_value};

pub fn lower_ast_if_statement_branch<K: DiagnosticSource>(
    branch: IfStatementBranch,
    local_ctx: Option<GlobalContextKey>,
    origin: &K,
) -> DiagResult<calsc_hir::ifs::IfStatementBranch> {
    match branch {
        IfStatementBranch::If { condition, body } => {
            let condition = lower_ast_value(ASTNode::clone(&condition), local_ctx.clone())?;
            let body = lower_ast_body(
                body.iter().map(|f| ASTNode::clone(f)).collect(),
                local_ctx,
                true,
                origin,
            )?;

            Ok(calsc_hir::ifs::IfStatementBranch::If { condition, body })
        }

        IfStatementBranch::IfElse { condition, body } => {
            let condition = lower_ast_value(ASTNode::clone(&condition), local_ctx.clone())?;
            let body = lower_ast_body(
                body.iter().map(|f| ASTNode::clone(f)).collect(),
                local_ctx,
                true,
                origin,
            )?;

            Ok(calsc_hir::ifs::IfStatementBranch::IfElse { condition, body })
        }

        IfStatementBranch::Else { body } => {
            let body = lower_ast_body(
                body.iter().map(|f| ASTNode::clone(f)).collect(),
                local_ctx,
                true,
                origin,
            )?;

            Ok(calsc_hir::ifs::IfStatementBranch::Else { body })
        }
    }
}

pub fn lower_ast_if_statement(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::IfStatement { branches } = node.kind.clone() {
        let mut hir_branches = vec![];

        for branch in branches {
            hir_branches.push(lower_ast_if_statement_branch(
                branch,
                local_ctx.clone(),
                &node,
            )?);
        }

        let node = HIRNode::new(
            HIRNodeKind::IfStatement {
                branches: hir_branches,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

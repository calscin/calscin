use std::hint::unreachable_unchecked;

use calsc_ast::{
    ifs::IfStatementBranch,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

use crate::{
    stage1::types::lower_ast_type,
    stage2::{funcs::lower_ast_body, values::lower_ast_value},
};

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

pub fn lower_ast_for_loop(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::ForLoop {
        iterator_type,
        iterator_name,
        iterated,
        body,
    } = node.kind.clone()
    {
        let iterator_type = lower_ast_type(iterator_type, &node, None)?;
        let iterated = lower_ast_value(ASTNode::clone(&iterated), local_ctx.clone())?;

        let variable_index = HIR_CONTEXT.with_borrow_mut(|f| {
            f.scope.mutate_entry(
                local_ctx.clone().unwrap(),
                |entry| {
                    entry.mutate_function(
                        |ff| {
                            ff.local_context
                                .as_mut()
                                .unwrap()
                                .introduce_variable_next_branch(
                                    iterator_name.clone(),
                                    iterator_type.clone(),
                                    true,
                                    &node,
                                )
                        },
                        &node,
                    )?
                },
                &node,
            )?
        })?;

        // Lower body after to avoid the branch introduction
        //and let the iterator be registered in the branch after the current one (aka the lower_ast_body introduced one)

        let body = lower_ast_body(
            body.iter().map(|f| ASTNode::clone(f)).collect(),
            local_ctx.clone(),
            true,
            &node,
        )?;

        let node = HIRNode::new(
            HIRNodeKind::ForLoop {
                iterator_type,
                iterator_name,
                iterator_variable_index: variable_index,
                iterated,
                body,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_while_loop(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::WhileLoop { condition, body } = node.kind.clone() {
        let condition = lower_ast_value(ASTNode::clone(&condition), local_ctx.clone())?;
        let body = lower_ast_body(
            body.iter().map(|f| ASTNode::clone(f)).collect(),
            local_ctx.clone(),
            true,
            &node,
        )?;

        let node = HIRNode::new(
            HIRNodeKind::WhileLoop { condition, body },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_loop(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::Loop { body } = node.kind.clone() {
        let body = lower_ast_body(
            body.iter().map(|f| ASTNode::clone(f)).collect(),
            local_ctx.clone(),
            true,
            &node,
        )?;

        let node = HIRNode::new(
            HIRNodeKind::Loop { body },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

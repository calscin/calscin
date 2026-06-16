use calsc_ast::{
    ASTContext,
    ifs::IfStatementBranch,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagResult, DiagnosticSource, diags::errors::build_internal_hir_node_leaked,
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{
    stage1::types::lower_ast_type,
    stage2::{funcs::lower_ast_body, values::lower_ast_value},
};

pub fn lower_ast_if_statement_branch<K: DiagnosticSource>(
    branch: IfStatementBranch,
    local_ctx: Option<GlobalContextKey>,
    origin: &K,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<calsc_hir::ifs::IfStatementBranch> {
    match branch {
        IfStatementBranch::If { condition, body } => {
            let condition = lower_ast_value(
                ASTNode::clone(&ast_ctx.nodes.get(&condition)),
                local_ctx.clone(),
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            let body = lower_ast_body(
                body.iter()
                    .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                    .collect(),
                local_ctx,
                true,
                origin,
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            Ok(calsc_hir::ifs::IfStatementBranch::If { condition, body })
        }

        IfStatementBranch::IfElse { condition, body } => {
            let condition = lower_ast_value(
                ASTNode::clone(&ast_ctx.nodes.get(&condition)),
                local_ctx.clone(),
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            let body = lower_ast_body(
                body.iter()
                    .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                    .collect(),
                local_ctx,
                true,
                origin,
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            Ok(calsc_hir::ifs::IfStatementBranch::IfElse { condition, body })
        }

        IfStatementBranch::Else { body } => {
            let body = lower_ast_body(
                body.iter()
                    .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                    .collect(),
                local_ctx,
                true,
                origin,
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            Ok(calsc_hir::ifs::IfStatementBranch::Else { body })
        }
    }
}

pub fn lower_ast_if_statement(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::IfStatement { branches } = node.kind.clone() {
        let mut hir_branches = vec![];

        for branch in branches {
            hir_branches.push(lower_ast_if_statement_branch(
                branch,
                local_ctx.clone(),
                &node,
                file_ctx,
                ctx,
                ast_ctx,
            )?);
        }

        let node = HIRNode::new(
            HIRNodeKind::IfStatement {
                branches: hir_branches,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_for_loop(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::ForLoop {
        iterator_type,
        iterator_name,
        iterated,
        body,
    } = node.kind.clone()
    {
        let iterator_type = lower_ast_type(iterator_type, &node, None, file_ctx, ctx)?;
        let iterated = lower_ast_value(
            ASTNode::clone(&ast_ctx.nodes.get(&iterated)),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let iterated_ref = ctx.nodes.get(&iterated).clone();

        let iterated = iterated_ref
            .use_as(
                iterator_type.clone(),
                iterated.clone(),
                None,
                local_ctx.clone(),
                ctx,
            )?
            .push(ctx);

        let variable_index = ctx.scope.mutate_entry(
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
                )
            },
            &node,
        )???;

        // Lower body after to avoid the branch introduction
        //and let the iterator be registered in the branch after the current one (aka the lower_ast_body introduced one)

        let body = lower_ast_body(
            body.iter()
                .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                .collect(),
            local_ctx.clone(),
            true,
            &node,
            file_ctx,
            ctx,
            ast_ctx,
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

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_while_loop(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::WhileLoop { condition, body } = node.kind.clone() {
        let condition = lower_ast_value(
            ASTNode::clone(&ast_ctx.nodes.get(&condition)),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let body = lower_ast_body(
            body.iter()
                .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                .collect(),
            local_ctx.clone(),
            true,
            &node,
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let node = HIRNode::new(
            HIRNodeKind::WhileLoop { condition, body },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_loop(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::Loop { body } = node.kind.clone() {
        let body = lower_ast_body(
            body.iter()
                .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                .collect(),
            local_ctx.clone(),
            true,
            &node,
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let node = HIRNode::new(
            HIRNodeKind::Loop { body },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

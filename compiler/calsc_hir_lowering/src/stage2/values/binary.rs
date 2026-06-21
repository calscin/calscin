use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind, BinaryOperator},
};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{
        build_expected_mutable, build_expected_type_error, build_internal_hir_node_leaked,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_typing::tree::Type;
use calsc_utils::alloc::arena::ArenaHandle;

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_binary_expression(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::BinaryExpression {
        left_expr,
        right_expr,
        operator,
    } = &node.kind
    {
        let left_expr = lower_ast_value(
            ast_ctx.nodes.get(left_expr).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let left_expr_ref = ctx.nodes.get(&left_expr).clone();

        let left_expr_type = left_expr_ref.get_type(local_ctx.clone(), ctx, Some(file_ctx))?;

        if left_expr_type == Type::Void || !left_expr_type.is_direct_numeric_generic() {
            return Err(build_expected_type_error(
                &"numeric".to_string(),
                &"".to_string(),
                &left_expr_ref,
            )
            .into());
        }

        let right_expr = lower_ast_value(
            ast_ctx.nodes.get(right_expr).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let right_expr_ref = ctx.nodes.get(&right_expr).clone();

        let right_expr = right_expr_ref
            .use_as(
                left_expr_type.clone(),
                right_expr.clone(),
                Some(left_expr.clone()),
                local_ctx.clone(),
                ctx,
                file_ctx,
            )?
            .push(ctx);

        if let BinaryOperator::Math(operator) = operator {
            if operator.assigns
                && !left_expr_ref.represents_mutable_variable(ctx, local_ctx, &node)?
            {
                return Err(build_expected_mutable(&left_expr_ref).into());
            }

            let node = HIRNode::new(
                HIRNodeKind::MathExpression {
                    left_expr,
                    right_expr,
                    operator: operator.clone(),
                },
                node.start.clone(),
                node.end.clone(),
            );

            return Ok(node.push(ctx));
        }

        if let BinaryOperator::Compare(operator) = operator {
            let node = HIRNode::new(
                HIRNodeKind::CompareExpression {
                    left_expr,
                    right_expr,
                    operator: operator.clone(),
                },
                node.start.clone(),
                node.end.clone(),
            );

            return Ok(node.push(ctx));
        }

        return Err(build_internal_hir_node_leaked(&node, &node).into());
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

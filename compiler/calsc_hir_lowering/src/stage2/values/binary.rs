use calsc_ast::nodes::{ASTNode, ASTNodeKind, BinaryOperator};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{
        build_expected_mutable, build_expected_type_error, build_internal_hir_node_leaked,
    },
};
use calsc_hir::{
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use calsc_typing::tree::Type;

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_binary_expression(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::BinaryExpression {
        left_expr,
        right_expr,
        operator,
    } = &node.kind
    {
        let left_expr = lower_ast_value(ASTNode::clone(left_expr), local_ctx.clone())?;
        let left_expr_type = left_expr.get_type(local_ctx.clone())?;

        if left_expr_type == Type::Void || !left_expr_type.is_direct_numeric_generic() {
            return Err(build_expected_type_error(
                &"numeric".to_string(),
                &"".to_string(),
                &*left_expr,
            )
            .into());
        }

        let right_expr = lower_ast_value(ASTNode::clone(right_expr), local_ctx.clone())?;
        let right_expr = right_expr
            .use_as(
                left_expr_type.clone(),
                right_expr.clone(),
                Some(left_expr.clone()),
                local_ctx,
            )?
            .push();

        if let BinaryOperator::Math(operator) = operator {
            if operator.assigns && !left_expr.represents_mutable_variable() {
                return Err(build_expected_mutable(&*left_expr).into());
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

            return Ok(node.push());
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

            return Ok(node.push());
        }

        return Err(build_internal_hir_node_leaked(&node, &node).into());
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

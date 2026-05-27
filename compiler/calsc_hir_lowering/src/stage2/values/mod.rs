//! Lowering for values

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagResult, diags::errors::build_expected_error};
use calsc_hir::{
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

use crate::stage2::{
    values::{
        booleans::lower_hir_inverse_condition,
        lits::lower_ast_literal,
        ptrs::{lower_ast_pointer_dereference, lower_ast_pointer_reference},
    },
    vars::lower_hir_variable_reference,
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

        ASTNodeKind::Range { .. } => lower_ast_range(node, local_ctx),

        ASTNodeKind::MathExpression { .. } => lower_ast_math_expression(node, local_ctx),
        ASTNodeKind::CompareExpression { .. } => lower_ast_compare_expression(node, local_ctx),

        ASTNodeKind::ElementReference(_) => lower_hir_variable_reference(node, local_ctx),

        _ => unsafe { unreachable_unchecked() },
    }
}

pub fn lower_ast_range(
    node: ASTNode,
    local_ctx: Option<&LocalContext>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::Range {
        start,
        end,
        increment,
    } = &node.kind
    {
        let start = lower_ast_value(ASTNode::clone(start), local_ctx)?;
        let start_type = start.get_type(local_ctx)?;

        if start_type.is_none() {
            return Err(build_expected_error(
                &"non void element".to_string(),
                &"void element".to_string(),
                &*start,
            )
            .into());
        }

        let start_type = start_type.unwrap();

        let end = lower_ast_value(ASTNode::clone(end), local_ctx)?
            .use_as(start_type.clone(), local_ctx)?
            .push();

        let mut incr = None;

        if increment.is_some() {
            incr = Some(lower_ast_value(
                ASTNode::clone(&increment.clone().unwrap()),
                local_ctx,
            )?);
        }

        let node = HIRNode::new(
            HIRNodeKind::Range {
                start,
                end,
                increment: incr,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_math_expression(
    node: ASTNode,
    local_ctx: Option<&LocalContext>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::MathExpression {
        left_expr,
        right_expr,
        operator,
    } = &node.kind
    {
        let left_expr = lower_ast_value(ASTNode::clone(left_expr), local_ctx)?;
        let left_expr_type = left_expr.get_type(local_ctx)?;

        if left_expr_type.is_none() || !left_expr_type.clone().unwrap().is_direct_numeric_generic()
        {
            return Err(build_expected_error(
                &"numeric type".to_string(),
                &"".to_string(),
                &*left_expr,
            )
            .into());
        }

        let left_expr_type = left_expr_type.unwrap();

        let right_expr = lower_ast_value(ASTNode::clone(right_expr), local_ctx)?
            .use_as(left_expr_type.clone(), local_ctx)?
            .push();

        let node = HIRNode::new(
            HIRNodeKind::MathExpression {
                left_expr,
                right_expr,
                operator: operator.clone(),
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_compare_expression(
    node: ASTNode,
    local_ctx: Option<&LocalContext>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::CompareExpression {
        left_expr,
        right_expr,
        operator,
    } = &node.kind
    {
        let left_expr = lower_ast_value(ASTNode::clone(left_expr), local_ctx)?;
        let left_expr_type = left_expr.get_type(local_ctx)?;

        if left_expr_type.is_none() || !left_expr_type.clone().unwrap().is_direct_numeric_generic()
        {
            return Err(build_expected_error(
                &"numeric type".to_string(),
                &"".to_string(),
                &*left_expr,
            )
            .into());
        }

        let left_expr_type = left_expr_type.unwrap();

        let right_expr = lower_ast_value(ASTNode::clone(right_expr), local_ctx)?
            .use_as(left_expr_type.clone(), local_ctx)?
            .push();

        let node = HIRNode::new(
            HIRNodeKind::CompareExpression {
                left_expr,
                right_expr,
                operator: operator.clone(),
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

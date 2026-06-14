//! Lowering for values

use std::collections::HashMap;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_internal_hir_node_leaked, build_unexpected_type_error},
};
use calsc_hir::{
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use calsc_typing::tree::Type;

use crate::stage2::{
    funcs::lower_ast_function_call,
    values::{
        binary::lower_ast_binary_expression,
        booleans::lower_hir_inverse_condition,
        index::lower_ast_index_usage,
        lits::lower_ast_literal,
        lru::lower_ast_lru,
        ptrs::{lower_ast_pointer_dereference, lower_ast_pointer_reference},
    },
    vars::lower_ast_variable_reference,
};

pub mod binary;
pub mod booleans;
pub mod index;
pub mod lits;
pub mod lru;
pub mod ptrs;

/// Lowers an AST value into an HIR value
pub fn lower_ast_value(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
) -> DiagResult<HIRArenaReference> {
    match node.kind {
        ASTNodeKind::IntLiteral(_)
        | ASTNodeKind::FloatLiteral(_)
        | ASTNodeKind::CharLiteral(_)
        | ASTNodeKind::StringLiteral(_)
        | ASTNodeKind::BooleanLiteral(_) => lower_ast_literal(node),

        ASTNodeKind::InverseCondition(_) => lower_hir_inverse_condition(node, local_ctx, file_ctx),

        ASTNodeKind::PointerReference(_) => lower_ast_pointer_reference(node, local_ctx, file_ctx),
        ASTNodeKind::PointerDereference(_) => {
            lower_ast_pointer_dereference(node, local_ctx, file_ctx)
        }

        ASTNodeKind::Range { .. } => lower_ast_range(node, local_ctx, file_ctx),

        ASTNodeKind::BinaryExpression { .. } => {
            lower_ast_binary_expression(node, local_ctx, file_ctx)
        }

        ASTNodeKind::FunctionCall { .. } => {
            lower_ast_function_call(node, None, local_ctx, file_ctx)
        }

        ASTNodeKind::ElementReference(_) => lower_ast_variable_reference(node, local_ctx),

        ASTNodeKind::StructLRUsage { .. } => lower_ast_lru(node, local_ctx, file_ctx),

        ASTNodeKind::StructuredInit { .. } => lower_ast_structured_init(node, local_ctx, file_ctx),

        ASTNodeKind::IndexUsage { .. } => lower_ast_index_usage(node, local_ctx, file_ctx),

        ASTNodeKind::ArrayInit(_) => lower_ast_array_init(node, local_ctx, file_ctx),

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    }
}

pub fn lower_ast_range(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::Range {
        start,
        end,
        increment,
    } = &node.kind
    {
        let start = lower_ast_value(ASTNode::clone(start), local_ctx.clone(), file_ctx)?;
        let start_type = start.get_type(local_ctx.clone())?;

        if start_type == Type::Void {
            return Err(build_unexpected_type_error(&Type::Void, &*start).into());
        }

        let end = lower_ast_value(ASTNode::clone(end), local_ctx.clone(), file_ctx)?;
        let end = end
            .use_as(
                start_type.clone(),
                end.clone(),
                Some(start.clone()),
                local_ctx.clone(),
            )?
            .push();

        let mut incr = None;

        if increment.is_some() {
            incr = Some(lower_ast_value(
                ASTNode::clone(&increment.clone().unwrap()),
                local_ctx,
                file_ctx,
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
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_structured_init(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::StructuredInit { values } = node.kind.clone() {
        let mut hir_values = HashMap::new();

        for (k, v) in values {
            hir_values.insert(
                k,
                lower_ast_value(ASTNode::clone(&v), local_ctx.clone(), file_ctx)?,
            );
        }

        let node = HIRNode::new(
            HIRNodeKind::StructuredInit { values: hir_values },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_array_init(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::ArrayInit(vals) = node.kind.clone() {
        let mut hir_vals = vec![];

        let first_val = lower_ast_value(ASTNode::clone(&vals[0]), local_ctx.clone(), file_ctx)?;
        let first_val_type = first_val.get_type(local_ctx.clone())?;

        hir_vals.push(first_val.clone());

        for i in 1..vals.len() {
            let val = lower_ast_value(ASTNode::clone(&vals[i]), local_ctx.clone(), file_ctx)?;

            // TODO: watch if using the other node parameter here actually impacts negatively
            let val = val
                .use_as(
                    first_val_type.clone(),
                    val.clone(),
                    Some(first_val.clone()),
                    local_ctx.clone(),
                )?
                .push();

            hir_vals.push(val);
        }

        let node = HIRNode::new(
            HIRNodeKind::ArrayInit { vals: hir_vals },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

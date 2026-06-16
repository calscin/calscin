//! Boolean values lowering

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    types::make_bool_type,
};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::stage2::values::lower_ast_value;

pub fn lower_hir_inverse_condition(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::InverseCondition(val) = node.kind.clone() {
        let val = lower_ast_value(
            ast_ctx.nodes.get(&val).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let val_ref = ctx.nodes.get(&val).clone();

        let val = val_ref.use_as(
            make_bool_type(&node, ctx, file_ctx),
            val.clone(),
            None,
            local_ctx.clone(),
            ctx,
            file_ctx,
        )?;

        let node = HIRNode::new(
            HIRNodeKind::InverseCondition(val.push(ctx)),
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

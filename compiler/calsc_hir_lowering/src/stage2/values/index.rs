use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_internal_hir_node_leaked, build_not_iterable},
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use calsc_typing::iter::IterableType;

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_index_usage(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::IndexUsage { val, index } = node.kind.clone() {
        let val = lower_ast_value(ASTNode::clone(&val), local_ctx.clone(), file_ctx, ctx)?;
        let val_type = val.get_type(local_ctx.clone(), ctx)?;

        if !val_type.is_iterable_at_all() {
            return Err(build_not_iterable(None, &val_type, &*val).into());
        }

        let index = lower_ast_value(ASTNode::clone(&index), local_ctx.clone(), file_ctx, ctx)?;
        let index = index
            .use_as(
                val_type.get_iterator_type(),
                index.clone(),
                None,
                local_ctx.clone(),
                ctx,
            )?
            .push(ctx);

        let output_type = val_type.get_iterator_output_type();

        let node = HIRNode::new(
            HIRNodeKind::IndexUsage {
                val,
                index,
                output_type,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

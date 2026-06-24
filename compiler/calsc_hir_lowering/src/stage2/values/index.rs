use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_internal_hir_node_leaked, build_not_iterable},
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};

use calsc_typing::traits::IterableType;
use calsc_utils::{alloc::arena::ArenaHandle, display_with_to_string};

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_index_usage(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::IndexUsage { val, index } = node.kind.clone() {
        let val = lower_ast_value(
            ast_ctx.nodes.get(&val).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let val_ref = ctx.nodes.get(&val).clone();
        let val_type = val_ref.get_type(local_ctx.clone(), ctx, Some(file_ctx))?;

        if !val_type.is_iterable_at_all(&ctx.type_ctx) {
            return Err(build_not_iterable(
                None,
                &display_with_to_string(&val_type, &ctx.type_ctx),
                &val_ref,
            )
            .into());
        }

        let index = lower_ast_value(
            ast_ctx.nodes.get(&index).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let index_ref = ctx.nodes.get(&index).clone();

        let index = index_ref
            .use_as(
                &val_type.get_iterator_type(&ctx.type_ctx),
                index.clone(),
                None,
                local_ctx.clone(),
                ctx,
                file_ctx,
            )?
            .push(ctx);

        let output_type = val_type.get_iterator_output_type(&ctx.type_ctx);

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

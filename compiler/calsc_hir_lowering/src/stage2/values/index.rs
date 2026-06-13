use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagResult, diags::errors::build_not_iterable};
use calsc_hir::{
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use calsc_typing::iter::IterableType;

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_index_usage(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::IndexUsage { val, index } = node.kind.clone() {
        let val = lower_ast_value(ASTNode::clone(&val), local_ctx.clone())?;
        let val_type = val.get_type(local_ctx.clone())?;

        if !val_type.is_iterable_at_all() {
            return Err(build_not_iterable(None, &val_type, &*val).into());
        }

        let index = lower_ast_value(ASTNode::clone(&index), local_ctx.clone())?;
        let index = index
            .use_as(
                val_type.get_iterator_type(),
                index.clone(),
                None,
                local_ctx.clone(),
            )?
            .push();

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

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

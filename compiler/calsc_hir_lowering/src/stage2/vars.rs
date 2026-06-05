//! Variable lowering

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagResult;
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

pub fn lower_hir_variable_reference(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::ElementReference(val) = &node.kind {
        let res = HIR_CONTEXT.with_borrow(|f| {
            f.scope
                .get_entry(curr_ctx.unwrap(), &node)
                .unwrap()
                .as_function(&node)
                .unwrap()
                .local_context
                .as_ref()
                .unwrap()
                .obtain(val.clone(), &node)
        });

        let ind = res?;

        let node = HIRNode::new(
            HIRNodeKind::VariableReference {
                name: val.clone(),
                variable_index: ind,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

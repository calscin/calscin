//! Variable lowering

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagResult;
use calsc_hir::{
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

pub fn lower_hir_variable_reference(
    node: ASTNode,
    curr_ctx: Option<&LocalContext>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::ElementReference(val) = &node.kind {
        let curr_ctx = curr_ctx.unwrap();

        let ind = curr_ctx.obtain(val.clone(), &node)?;

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

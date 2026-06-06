//! Boolean values lowering

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagResult;
use calsc_hir::{
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
    types::make_bool_type,
};

use crate::stage2::values::lower_ast_value;

pub fn lower_hir_inverse_condition(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::InverseCondition(val) = node.kind.clone() {
        let val = lower_ast_value(ASTNode::clone(&val), local_ctx.clone())?;
        let val = val.use_as(make_bool_type(&node), val.clone(), None, local_ctx.clone())?;

        let node = HIRNode::new(
            HIRNodeKind::InverseCondition(val.push()),
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

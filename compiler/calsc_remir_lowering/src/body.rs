use calsc_diagnostics::DiagPossible;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::module::Module;

use crate::funcs::lower_hir_function_call;

pub fn lower_hir_body_node(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    match &node.kind {
        HIRNodeKind::FunctionCall { .. } => {
            let _ = lower_hir_function_call(node, ctx, module)?;
            Ok(())
        }

        e => panic!("Unexpected {:#?}", e),
    }
}

pub fn lower_hir_body(
    nodes: Vec<HIRArenaReference>,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    for node in nodes {
        lower_hir_body_node(node, ctx, module)?;
    }

    Ok(())
}

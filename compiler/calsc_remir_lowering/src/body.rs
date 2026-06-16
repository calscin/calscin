use calsc_diagnostics::DiagPossible;
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::module::Module;

use crate::{
    control::{fors::lower_hir_for_loop, ifs::lower_hir_if_statement, loops::lower_hir_while_loop},
    funcs::{lower_hir_function_call, lower_hir_function_return},
    values::lower_hir_value,
    vars::{lower_hir_variable_assign, lower_hir_variable_declaration},
};

pub fn lower_hir_body_node(
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagPossible {
    let node_ref = hirctx.nodes.get(&node);

    match &node_ref.kind {
        HIRNodeKind::FunctionCall { .. } => {
            let _ = lower_hir_function_call(node, ctx, module, hirctx)?;
            Ok(())
        }

        HIRNodeKind::VariableDeclaration { .. } => {
            lower_hir_variable_declaration(node, ctx, module, hirctx)
        }

        HIRNodeKind::Assignment { .. } => lower_hir_variable_assign(node, ctx, module, hirctx),

        HIRNodeKind::IfStatement { .. } => lower_hir_if_statement(node, ctx, module, hirctx),
        HIRNodeKind::ForLoop { .. } => lower_hir_for_loop(node, ctx, module, hirctx),
        HIRNodeKind::WhileLoop { .. } => lower_hir_while_loop(node, ctx, module, hirctx),

        HIRNodeKind::ReturnStatement { .. } => lower_hir_function_return(node, ctx, module, hirctx),

        _ => {
            let _ = lower_hir_value(node, ctx, module, hirctx)?;

            Ok(())
        }
    }
}

pub fn lower_hir_body(
    nodes: Vec<ArenaHandle>,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagPossible {
    for node in nodes {
        lower_hir_body_node(node, ctx, module, hirctx)?;
    }

    Ok(())
}

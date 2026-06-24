use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    block::sync::VariableSynchronizer,
    builders::{build_conditional_branch, build_unconditional_branch},
    module::Module,
    values::int::SSAIntValue,
    writer::InstructionWriter,
};

use crate::{body::lower_hir_body, result::CalscinRemirResult, values::lower_hir_value};

pub fn lower_hir_while_loop(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &mut HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node).clone();

    if let HIRNodeKind::WhileLoop { condition, body } = node_ref.kind.clone() {
        // We use the following technique to lower a while loop:
        // - A loop header block that contains the Phi code for values and condition checking
        // - A body block that contains the body
        // - An exit / merge block
        //
        // Before starting, we create the merge block
        // We first set the sync pos on the current block.
        // We then create the loop header block without filling it
        // We then create the body block and fill it.
        // We then fill the header block and merge SSA variables in it using Phi nodes helpers.
        // We then set our current position to the merge block.

        // Create the merge block
        let merge_block = module
            .create_block("merge_block".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        // Setting the sync point
        module.set_sync_point(module.pos_block.clone().unwrap());

        // Create the header block
        let header_block = module
            .create_block("while_header_block".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        // Create the body block
        let body_block = module
            .create_block("while_body".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        // Filling the body block
        module.move_end(body_block.clone(), module.pos_function.clone().unwrap());

        lower_hir_body(body, local_ctx, module, ctx)?;

        // Build the unconditional branch jump to header
        build_unconditional_branch(module, header_block.clone());

        // Filling the header block
        module.move_end(header_block.clone(), module.pos_function.clone().unwrap());

        // Write condition and branch
        {
            let condition = lower_hir_value(condition, local_ctx, module, ctx)?;
            let condition: SSAIntValue = condition
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            build_conditional_branch(module, condition, body_block, merge_block.clone())
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        }

        // Set current pos to merge block
        module.move_end(merge_block, module.pos_function.clone().unwrap());

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into());
    }
}

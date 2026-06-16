use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    block::{Block, sync::VariableSynchronizer, vars::BlockVariable},
    builders::{
        build_alloca, build_conditional_branch, build_const_int, build_int_compare,
        build_math_op_int, build_unconditional_branch,
    },
    misc::{CompareOperator, MathOperator},
    module::Module,
    values::int::SSAIntValue,
    writer::InstructionWriter,
};

use crate::{
    body::lower_hir_body, range::lower_hir_range, result::CalscinRemirResult, types::lower_type,
};

#[allow(unsafe_code)]
pub fn lower_hir_for_loop(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::ForLoop {
        iterator_type,
        iterator_name,
        iterator_variable_index: _,
        iterated,
        body,
    } = node_ref.kind.clone()
    {
        let iterated = lower_hir_range(iterated, local_ctx, module, ctx)?;
        let iterator_type = lower_type(iterator_type)?;

        // We use the following technique to lower a for loop:
        // - A loop header block that contains the Phi code for the iterator index and condition
        // - A body block that contains the body and also contains the incrementation
        // - An exit / merge block
        //
        // Before starting, we create the merge block
        // We first create the iterator variable on the current block and set the sync pos on the current block.
        // Then, we create the loop header block without filling it.
        // We then create the body block and fill it.
        // We then fill the header block and merge SSA variables in it using the Phi nodes helper.
        // We then set our current position to the merge block

        let curr_pos = module.pos_block.clone().unwrap();

        // Setting the sync point
        module.set_sync_point(curr_pos.clone());

        // Create the merge block
        let merge_block = module
            .create_block("merge_block".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        // Append the variable
        // Creation of the iterator variable
        let variable_ptr_size = build_const_int(module, 0, 64, false)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;
        let variable_pointer = build_alloca(module, variable_ptr_size, Some(iterator_type))
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let mut variable =
            BlockVariable::new_pointer(String::clone(&iterator_name), variable_pointer);
        variable
            .write(module, iterated.start.clone().into())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        module.blocks[curr_pos.id].append_variable(variable); // Appends the variable

        // Creating the loop header block
        let header_block = module
            .create_block("for_header".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        build_unconditional_branch(module, header_block.clone()); // Jump to the header block from the original block

        module.set_sync_point(header_block.clone());

        // Creating the body block
        let body_block = module
            .create_block("for_body".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        // Filling the body block
        module.move_end(body_block.clone(), module.pos_function.clone().unwrap());

        lower_hir_body(body, local_ctx, module, ctx)?;

        // Increment the iterator
        {
            let variable = module.blocks[body_block.id].variables[&*iterator_name].clone(); // Clones the variable. 
            //This is fine since the BlockVariable instance that is cloned will be discarded at the end of the block and doesn't escape

            let iterator_value = variable
                .read(module)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let iterator_value: SSAIntValue = iterator_value
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let new_value = build_math_op_int(
                module,
                iterator_value,
                iterated.increment.clone(),
                MathOperator::Add,
                iterated.increment.signed,
                true,
                true,
                false,
            )
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

            // Tricky hack to avoid double borrowing of module
            // This is normally safe as the block reference doesn't escape this block and isn't stored
            let block = unsafe {
                std::mem::transmute::<&mut Block, &'static mut Block>(
                    &mut module.blocks[body_block.id],
                )
            };

            block
                .variables
                .get_mut(&*iterator_name)
                .unwrap()
                .write(module, new_value.into())
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        }

        // Build the unconditional branch jump to header
        build_unconditional_branch(module, header_block.clone());

        // Filling the header block
        module.move_end(header_block.clone(), module.pos_function.clone().unwrap());

        // Write condition and branch
        {
            // Tricky hack to avoid double borrowing of module
            // This is normally safe as the block reference doesn't escape this block and isn't stored
            let block = unsafe {
                std::mem::transmute::<&mut Block, &'static mut Block>(
                    &mut module.blocks[header_block.id],
                )
            };

            let value = block.variables[&*iterator_name]
                .read(module)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let value: SSAIntValue = value
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let condition = build_int_compare(
                module,
                value,
                iterated.end.clone(),
                CompareOperator::Lt,
                iterated.end.signed,
            )
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

            build_conditional_branch(module, condition, body_block, merge_block.clone())
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        }

        // Set current pos to merge block
        module.move_end(merge_block, module.pos_function.clone().unwrap());

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(node_ref, node_ref).into());
    }
}

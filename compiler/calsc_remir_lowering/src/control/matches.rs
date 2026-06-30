use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_typing::{allocs::ENUM_CONTAINER_ALLOC, types::primitive::PrimitiveType};
use calsc_utils::{alloc::arena::ArenaHandle, vec_contains};
use remir::{
    block::{BlockReference, sync::VariableSynchronizer},
    builders::{
        build_const_int, build_extract_value, build_load, build_struct_gep, build_switch,
        build_unconditional_branch,
    },
    module::Module,
    values::{ValueType, int::SSAIntValue, ptr::SSAPointerValue, structs::SSAStructValue},
    writer::InstructionWriter,
};

use crate::{body::lower_hir_body, result::CalscinRemirResult, values::lower_hir_value};

pub fn lower_hir_match_block(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &mut HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node).clone();

    if let HIRNodeKind::MatchBlock {
        val,
        matches,
        default_match,
    } = node_ref.kind.clone()
    {
        module.set_sync_point(module.pos_block.clone().unwrap());

        let original_block = module.pos_block.clone().unwrap();

        let mut blocks: Vec<(SSAIntValue, BlockReference)> = vec![];

        let merge_block = module
            .create_block("match_merge".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let default_block = if default_match.is_some() {
            module
                .create_block("match_default".to_string())
                .convert(node_ref.start.clone(), node_ref.end.clone())?
        } else {
            merge_block.clone()
        };

        // We first convert the value and obtain the marker

        let val = lower_hir_value(val, local_ctx, module, ctx)?;
        let marker: SSAIntValue;

        if let ValueType::Pointer(_) = val.value_type {
            let val: SSAPointerValue = val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let marker_ptr = build_struct_gep(module, val, 0)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let marker_val = build_load(module, marker_ptr)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            marker = marker_val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        } else {
            let val: SSAStructValue = val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let marker_val = build_extract_value(module, val, 0)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            marker = marker_val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        }

        // We then create the blocks
        for branch in matches {
            if let PrimitiveType::EnumEntry(container, name) = branch.0 {
                let (marker, sz) = ENUM_CONTAINER_ALLOC.with(|f| {
                    (
                        vec_contains(&f.borrow().get(&container).entries_order, &name).unwrap(),
                        f.borrow()
                            .get(&container)
                            .get_marker_type()
                            .as_primitive()
                            .size
                            .0,
                    )
                });

                let inner_marker = build_const_int(module, marker as i128, sz, false)
                    .convert(node_ref.start.clone(), node_ref.end.clone())?;

                let block = module
                    .create_block(format!("match_{}", marker))
                    .convert(node_ref.start.clone(), node_ref.end.clone())?;

                module.move_end(block.clone(), module.pos_function.clone().unwrap());

                lower_hir_body(branch.3, local_ctx, module, ctx)?;

                build_unconditional_branch(module, merge_block.clone());

                blocks.push((inner_marker, block))
            } else {
                panic!("Match inner type is not enum entry!")
            }
        }

        if default_match.is_some() {
            let default_match = default_match.unwrap();

            module.move_end(default_block.clone(), module.pos_function.clone().unwrap());

            lower_hir_body(default_match, local_ctx, module, ctx)?;

            build_unconditional_branch(module, merge_block.clone());
        }

        module.stop_sync_point();

        module.move_end(original_block, module.pos_function.clone().unwrap());

        build_switch(module, marker, default_block, blocks)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        module.move_end(merge_block, module.pos_function.clone().unwrap());

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into());
    }
}

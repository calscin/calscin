use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, ifs::IfStatementBranch, localctx::LocalContext, nodes::HIRNodeKind};

use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    block::{BlockReference, sync::VariableSynchronizer},
    builders::{build_conditional_branch, build_unconditional_branch},
    module::Module,
    values::int::SSAIntValue,
    writer::InstructionWriter,
};

use crate::{body::lower_hir_body, result::CalscinRemirResult, values::lower_hir_value};

pub fn lower_hir_if_statement_node_branches(
    branch: IfStatementBranch,
    module: &mut Module,
    node: &ArenaHandle,
    branch_blocks: &mut Vec<BlockReference>,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(node);

    match branch {
        IfStatementBranch::If { .. } => {
            let body_block = module
                .create_block("if".to_string())
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            branch_blocks.push(body_block);

            Ok(())
        }

        IfStatementBranch::IfElse { .. } => {
            let cond_block = module
                .create_block("ifelse_cond".to_string())
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let body_block = module
                .create_block("ifelse_body".to_string())
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            branch_blocks.push(cond_block);
            branch_blocks.push(body_block);

            Ok(())
        }

        IfStatementBranch::Else { .. } => {
            let body_block = module
                .create_block("else_body".to_string())
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            branch_blocks.push(body_block);

            Ok(())
        }
    }
}

pub fn lower_hir_if_statement_branch(
    branch: IfStatementBranch,
    local_ctx: &LocalContext,
    module: &mut Module,
    ind: &mut usize,
    node: &ArenaHandle,
    merge_ref: &BlockReference,
    branch_blocks: &Vec<BlockReference>,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(node);

    match branch {
        IfStatementBranch::If { condition, body } => {
            let condition = lower_hir_value(condition, local_ctx, module, ctx)?;
            let condition = SSAIntValue::try_from(condition)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            build_conditional_branch(
                module,
                condition,
                branch_blocks[*ind].clone(),
                branch_blocks[*ind + 1].clone(),
            )
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            lower_hir_body(body, local_ctx, module, ctx)?;

            build_unconditional_branch(module, merge_ref.clone());

            *ind += 1;

            Ok(())
        }

        IfStatementBranch::IfElse { condition, body } => {
            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            let condition = lower_hir_value(condition, local_ctx, module, ctx)?;
            let condition = SSAIntValue::try_from(condition)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            build_conditional_branch(
                module,
                condition,
                branch_blocks[*ind + 1].clone(),
                branch_blocks[*ind + 2].clone(),
            )
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

            *ind += 1;

            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            lower_hir_body(body, local_ctx, module, ctx)?;

            build_unconditional_branch(module, merge_ref.clone());

            Ok(())
        }

        IfStatementBranch::Else { body } => {
            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            lower_hir_body(body, local_ctx, module, ctx)?;

            build_unconditional_branch(module, merge_ref.clone());

            Ok(())
        }
    }
}

pub fn lower_hir_if_statement(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::IfStatement { branches } = node_ref.kind.clone() {
        module.set_sync_point(module.pos_block.clone().unwrap());

        let merge_block = module
            .create_block("merge".to_string())
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let mut branch_blocks = vec![];

        for branch in branches.clone() {
            lower_hir_if_statement_node_branches(branch, module, &node, &mut branch_blocks, ctx)?;
        }

        branch_blocks.push(merge_block.clone());

        let mut ind = 0;
        for branch in branches {
            lower_hir_if_statement_branch(
                branch,
                local_ctx,
                module,
                &mut ind,
                &node,
                &merge_block,
                &branch_blocks,
                ctx,
            )?;
        }

        module.stop_sync_point();

        module.move_end(merge_block, module.pos_function.clone().unwrap());

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(node_ref, node_ref).into());
    }
}

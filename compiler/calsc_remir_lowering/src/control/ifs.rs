use std::hint::unreachable_unchecked;

use calsc_diagnostics::DiagPossible;
use calsc_hir::{
    ifs::IfStatementBranch,
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use rand::random;
use remir::{
    block::BlockReference,
    builders::{build_conditional_branch, build_unconditional_branch},
    module::Module,
    values::int::SSAIntValue,
    writer::InstructionWriter,
};

use crate::{
    body::lower_hir_body, generate_block_seed, result::CalscinRemirResult, values::lower_hir_value,
};

pub fn lower_hir_if_statement_node_branches(
    branch: IfStatementBranch,
    local_ctx: &LocalContext,
    module: &mut Module,
    seed: String,
    node: &HIRArenaReference,
    branch_blocks: &mut Vec<BlockReference>,
) -> DiagPossible {
    match branch {
        IfStatementBranch::If { .. } => {
            let body_block = module
                .create_block(format!("{}_if", seed))
                .convert(node.start.clone(), node.end.clone())?;

            branch_blocks.push(body_block);

            Ok(())
        }

        IfStatementBranch::IfElse { .. } => {
            let inner_seed = generate_block_seed();

            let cond_block = module
                .create_block(format!("{}__{}_ifelse_cond", seed, inner_seed))
                .convert(node.start.clone(), node.end.clone())?;

            let body_block = module
                .create_block(format!("{}__{}_ifelse_body", seed, inner_seed))
                .convert(node.start.clone(), node.end.clone())?;

            branch_blocks.push(cond_block);
            branch_blocks.push(body_block);

            Ok(())
        }

        IfStatementBranch::Else { .. } => {
            let body_block = module
                .create_block(format!("{}_else", seed))
                .convert(node.start.clone(), node.end.clone())?;

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
    node: &HIRArenaReference,
    merge_ref: &BlockReference,
    branch_blocks: &Vec<BlockReference>,
) -> DiagPossible {
    match branch {
        IfStatementBranch::If { condition, body } => {
            let condition = lower_hir_value(condition, local_ctx, module)?;
            let condition =
                SSAIntValue::try_from(condition).convert(node.start.clone(), node.end.clone())?;

            build_conditional_branch(
                module,
                condition,
                branch_blocks[*ind].clone(),
                branch_blocks[*ind + 1].clone(),
            )
            .convert(node.start.clone(), node.end.clone())?;

            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            lower_hir_body(body, local_ctx, module)?;

            build_unconditional_branch(module, merge_ref.clone());

            *ind += 1;

            Ok(())
        }

        IfStatementBranch::IfElse { condition, body } => {
            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            let condition = lower_hir_value(condition, local_ctx, module)?;
            let condition =
                SSAIntValue::try_from(condition).convert(node.start.clone(), node.end.clone())?;

            build_conditional_branch(
                module,
                condition,
                branch_blocks[*ind + 1].clone(),
                branch_blocks[*ind + 2].clone(),
            )
            .convert(node.start.clone(), node.end.clone())?;

            *ind += 1;

            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            lower_hir_body(body, local_ctx, module)?;

            build_unconditional_branch(module, merge_ref.clone());

            Ok(())
        }

        IfStatementBranch::Else { body } => {
            module.move_end(
                branch_blocks[*ind].clone(),
                module.pos_function.clone().unwrap(),
            );

            lower_hir_body(body, local_ctx, module)?;

            build_unconditional_branch(module, merge_ref.clone());

            Ok(())
        }
    }
}

pub fn lower_hir_if_statement(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::IfStatement { branches } = node.kind.clone() {
        let seed = generate_block_seed();

		let merge_block = 

        let mut branch_blocks = vec![];

        for branch in branches.clone() {
            lower_hir_if_statement_node_branches(
                branch,
                local_ctx,
                module,
                seed.clone(),
                &node,
                &mut branch_blocks,
            )?;
        }

        branch_blocks.push(value);

        todo!()
    } else {
        unsafe { unreachable_unchecked() }
    }
}

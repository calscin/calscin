use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{builders::build_const_int, module::Module, values::int::SSAIntValue};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

pub struct MIRRange {
    pub start: SSAIntValue,
    pub end: SSAIntValue,
    pub increment: SSAIntValue,
}

pub fn lower_hir_range(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagResult<MIRRange> {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::Range {
        start,
        end,
        increment,
    } = node_ref.kind.clone()
    {
        let start: SSAIntValue = lower_hir_value(start, local_ctx, module, ctx)?
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let end: SSAIntValue = lower_hir_value(end, local_ctx, module, ctx)?
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let incr: SSAIntValue;

        if increment.is_none() {
            incr = build_const_int(module, 1, start.size, start.signed)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        } else {
            incr = lower_hir_value(increment.unwrap(), local_ctx, module, ctx)?
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        }

        Ok(MIRRange {
            start,
            end,
            increment: incr,
        })
    } else {
        return Err(build_internal_hir_node_leaked(&*node_ref, &*node_ref).into());
    }
}

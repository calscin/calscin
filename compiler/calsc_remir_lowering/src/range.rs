use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{builders::build_const_int, module::Module, values::int::SSAIntValue};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

pub struct MIRRange {
    pub start: SSAIntValue,
    pub end: SSAIntValue,
    pub increment: SSAIntValue,
}

pub fn lower_hir_range(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<MIRRange> {
    if let HIRNodeKind::Range {
        start,
        end,
        increment,
    } = node.kind.clone()
    {
        let start: SSAIntValue = lower_hir_value(start, local_ctx, module)?
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let end: SSAIntValue = lower_hir_value(end, local_ctx, module)?
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let incr: SSAIntValue;

        if increment.is_none() {
            incr = build_const_int(module, 1, start.size, start.signed)
                .convert(node.start.clone(), node.end.clone())?;
        } else {
            incr = lower_hir_value(increment.unwrap(), local_ctx, module)?
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;
        }

        Ok(MIRRange {
            start,
            end,
            increment: incr,
        })
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

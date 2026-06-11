use std::hint::unreachable_unchecked;

use calsc_diagnostics::DiagResult;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    builders::{build_array_gep, build_load},
    module::Module,
    values::{BaseSSAValue, int::SSAIntValue},
};

use crate::{
    reads::lower_hir_readable_pointer, result::CalscinRemirResult, values::lower_hir_value,
};

pub fn lower_hir_index_usage(
    node: HIRArenaReference,
    local_context: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    if let HIRNodeKind::IndexUsage {
        val,
        index,
        output_type: _,
    } = node.kind.clone()
    {
        let val = lower_hir_readable_pointer(val, local_context, module)?;
        let index = lower_hir_value(index, local_context, module)?;

        let index: SSAIntValue = index
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let ptr =
            build_array_gep(module, val, index).convert(node.start.clone(), node.end.clone())?;

        let val = build_load(module, ptr).convert(node.start.clone(), node.end.clone())?;

        Ok(val)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

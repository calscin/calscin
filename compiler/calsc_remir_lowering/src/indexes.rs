use std::hint::unreachable_unchecked;

use calsc_diagnostics::DiagResult;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    builders::{build_gep, build_load},
    module::Module,
    values::{BaseSSAValue, ValueType, int::SSAIntValue, ptr::SSAPointerValue},
};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

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
        let val = lower_hir_value(val, local_context, module)?;
        let index = lower_hir_index_usage(index, local_context, module)?;

        let index: SSAIntValue = index
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        if let ValueType::Array(_) = &val.value_type {
            todo!("Raw array indexing is not supported yet!")
        }

        let val: SSAPointerValue = val
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let ptr = build_gep(module, val, index).convert(node.start.clone(), node.end.clone())?;

        let val = build_load(module, ptr).convert(node.start.clone(), node.end.clone())?;

        Ok(val)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

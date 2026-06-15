use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
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
    hirctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    if let HIRNodeKind::IndexUsage {
        val,
        index,
        output_type: _,
    } = node.kind.clone()
    {
        let val = lower_hir_readable_pointer(val, local_context, module, hirctx)?;
        let index = lower_hir_value(index, local_context, module, hirctx)?;

        let index: SSAIntValue = index
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let ptr =
            build_array_gep(module, val, index).convert(node.start.clone(), node.end.clone())?;

        let val = build_load(module, ptr).convert(node.start.clone(), node.end.clone())?;

        Ok(val)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

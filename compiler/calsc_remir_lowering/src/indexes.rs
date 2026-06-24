use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    builders::{build_array_gep, build_load},
    module::Module,
    values::{BaseSSAValue, int::SSAIntValue},
};

use crate::{
    reads::lower_hir_readable_pointer, result::CalscinRemirResult, values::lower_hir_value,
};

pub fn lower_hir_index_usage(
    node: ArenaHandle,
    local_context: &LocalContext,
    module: &mut Module,
    hirctx: &mut HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = hirctx.nodes.get(&node).clone();

    if let HIRNodeKind::IndexUsage {
        val,
        index,
        output_type: _,
    } = node_ref.kind.clone()
    {
        let val = lower_hir_readable_pointer(val, local_context, module, hirctx)?;
        let index = lower_hir_value(index, local_context, module, hirctx)?;

        let index: SSAIntValue = index
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let ptr = build_array_gep(module, val, index)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let val = build_load(module, ptr).convert(node_ref.start.clone(), node_ref.end.clone())?;

        Ok(val)
    } else {
        return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into());
    }
}

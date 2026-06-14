use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_expected_referencable, build_internal_hir_node_leaked},
};
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    builders::build_load,
    module::Module,
    values::{BaseSSAValue, ptr::SSAPointerValue},
};

use crate::{
    result::CalscinRemirResult, values::lower_hir_value, vars::lower_hir_variable_reference,
};

pub fn lower_hir_pointer_reference(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    if let HIRNodeKind::PointerReference(inner) = node.kind.clone() {
        let inner = lower_hir_variable_reference(inner, local_ctx, module)?;

        if !inner.write_as_pointer {
            return Err(build_expected_referencable(&*node).into());
        }

        Ok(inner.held_value.clone().unwrap())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

pub fn lower_hir_pointer_dereference(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    if let HIRNodeKind::PointerDereference(inner) = node.kind.clone() {
        let inner = lower_hir_value(inner, local_ctx, module)?;
        let inner =
            SSAPointerValue::try_from(inner).convert(node.start.clone(), node.end.clone())?;

        // TODO: try using variable wrappers when possible

        let res = build_load(module, inner).convert(node.start.clone(), node.end.clone())?;

        Ok(res)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

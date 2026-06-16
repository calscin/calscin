use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_expected_referencable, build_internal_hir_node_leaked},
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    builders::build_load,
    module::Module,
    values::{BaseSSAValue, ptr::SSAPointerValue},
};

use crate::{
    result::CalscinRemirResult, values::lower_hir_value, vars::lower_hir_variable_reference,
};

pub fn lower_hir_pointer_reference(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::PointerReference(inner) = node_ref.kind.clone() {
        let inner = lower_hir_variable_reference(inner, local_ctx, module, ctx)?;

        if !inner.write_as_pointer {
            return Err(build_expected_referencable(node_ref).into());
        }

        Ok(inner.held_value.clone().unwrap())
    } else {
        return Err(build_internal_hir_node_leaked(node_ref, node_ref).into());
    }
}

pub fn lower_hir_pointer_dereference(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = hirctx.nodes.get(&node);

    if let HIRNodeKind::PointerDereference(inner) = node_ref.kind.clone() {
        let inner = lower_hir_value(inner, local_ctx, module, hirctx)?;
        let inner = SSAPointerValue::try_from(inner)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        // TODO: try using variable wrappers when possible

        let res =
            build_load(module, inner).convert(node_ref.start.clone(), node_ref.end.clone())?;

        Ok(res)
    } else {
        return Err(build_internal_hir_node_leaked(node_ref, node_ref).into());
    }
}

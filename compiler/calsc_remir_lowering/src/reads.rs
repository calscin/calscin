use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_internal_hir_node_leaked, build_remir_error},
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    builders::{build_array_gep, build_struct_gep},
    module::Module,
    values::{ValueType, int::SSAIntValue, ptr::SSAPointerValue},
};

use crate::{
    result::CalscinRemirResult, values::lower_hir_value, vars::lower_hir_variable_reference,
};

pub fn lower_hir_readable_pointer(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagResult<SSAPointerValue> {
    let node_ref = ctx.nodes.get(&node);

    match node_ref.kind {
        HIRNodeKind::VariableReference { .. } => {
            let var = lower_hir_variable_reference(node.clone(), local_ctx, module, ctx)?;

            if !var.write_as_pointer {
                return Err(build_remir_error(
                    &"variable is not a pointer but must be!".to_string(),
                    node_ref.start.clone(),
                    node_ref.end.clone(),
                )
                .into());
            }

            Ok(var
                .held_value
                .clone()
                .unwrap()
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?)
        }
        HIRNodeKind::FieldReference { .. } => {
            lower_hir_readable_field(node, local_ctx, module, ctx)
        }
        HIRNodeKind::PointerDereference(_) => {
            lower_hir_readable_pointer_deref(node, local_ctx, module, ctx)
        }

        HIRNodeKind::IndexUsage { .. } => {
            lower_hir_readable_index_usage(node, local_ctx, module, ctx)
        }

        _ => panic!(),
    }
}

pub fn lower_hir_readable_field(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagResult<SSAPointerValue> {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::FieldReference {
        val,
        field_ind,
        name: _,
    } = node_ref.kind.clone()
    {
        let val = lower_hir_value(val, local_ctx, module, ctx)?;

        if let ValueType::Struct(_) = &val.value_type {
            return Err(build_remir_error(
                &"variable is not a pointer but must be!",
                node_ref.start.clone(),
                node_ref.end.clone(),
            )
            .into());
        }

        let val: SSAPointerValue = val
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let ptr = build_struct_gep(module, val, field_ind)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        Ok(ptr)
    } else {
        return Err(build_internal_hir_node_leaked(&*node_ref, &*node_ref).into());
    }
}

pub fn lower_hir_readable_pointer_deref(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagResult<SSAPointerValue> {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::PointerDereference(inner) = node_ref.kind.clone() {
        let inner = lower_hir_value(inner, local_ctx, module, ctx)?;

        let inner: SSAPointerValue = inner
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        Ok(inner)
    } else {
        return Err(build_internal_hir_node_leaked(&*node_ref, &*node_ref).into());
    }
}

pub fn lower_hir_readable_index_usage(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagResult<SSAPointerValue> {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::IndexUsage {
        val,
        index,
        output_type: _,
    } = node_ref.kind.clone()
    {
        let val = lower_hir_value(val, local_ctx, module, ctx)?;
        let val: SSAPointerValue = val
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let index = lower_hir_value(index, local_ctx, module, ctx)?;
        let index: SSAIntValue = index
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let ptr = build_array_gep(module, val, index)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        Ok(ptr)
    } else {
        return Err(build_internal_hir_node_leaked(&*node_ref, &*node_ref).into());
    }
}

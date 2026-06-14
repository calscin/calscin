use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_internal_hir_node_leaked, build_remir_error},
};
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    builders::{build_array_gep, build_struct_gep},
    module::Module,
    values::{ValueType, int::SSAIntValue, ptr::SSAPointerValue},
};

use crate::{
    result::CalscinRemirResult, values::lower_hir_value, vars::lower_hir_variable_reference,
};

pub fn lower_hir_readable_pointer(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<SSAPointerValue> {
    match node.kind {
        HIRNodeKind::VariableReference { .. } => {
            let var = lower_hir_variable_reference(node.clone(), local_ctx, module)?;

            if !var.write_as_pointer {
                return Err(build_remir_error(
                    &"variable is not a pointer but must be!".to_string(),
                    node.start.clone(),
                    node.end.clone(),
                )
                .into());
            }

            Ok(var
                .held_value
                .clone()
                .unwrap()
                .try_into()
                .convert(node.start.clone(), node.end.clone())?)
        }

        HIRNodeKind::FieldReference { .. } => lower_hir_readable_field(node, local_ctx, module),
        HIRNodeKind::PointerDereference(_) => {
            lower_hir_readable_pointer_deref(node, local_ctx, module)
        }

        HIRNodeKind::IndexUsage { .. } => lower_hir_readable_index_usage(node, local_ctx, module),

        _ => panic!(),
    }
}

pub fn lower_hir_readable_field(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<SSAPointerValue> {
    if let HIRNodeKind::FieldReference {
        val,
        field_ind,
        name: _,
    } = node.kind.clone()
    {
        let val = lower_hir_value(val, local_ctx, module)?;

        if let ValueType::Struct(_) = &val.value_type {
            return Err(build_remir_error(
                &"variable is not a pointer but must be!",
                node.start.clone(),
                node.end.clone(),
            )
            .into());
        }

        let val: SSAPointerValue = val
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let ptr = build_struct_gep(module, val, field_ind)
            .convert(node.start.clone(), node.end.clone())?;

        Ok(ptr)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

pub fn lower_hir_readable_pointer_deref(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<SSAPointerValue> {
    if let HIRNodeKind::PointerDereference(inner) = node.kind.clone() {
        let inner = lower_hir_value(inner, local_ctx, module)?;

        let inner: SSAPointerValue = inner
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        Ok(inner)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

pub fn lower_hir_readable_index_usage(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<SSAPointerValue> {
    if let HIRNodeKind::IndexUsage {
        val,
        index,
        output_type: _,
    } = node.kind.clone()
    {
        let val = lower_hir_value(val, local_ctx, module)?;
        let val: SSAPointerValue = val
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let index = lower_hir_value(index, local_ctx, module)?;
        let index: SSAIntValue = index
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let ptr =
            build_array_gep(module, val, index).convert(node.start.clone(), node.end.clone())?;

        Ok(ptr)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

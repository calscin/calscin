use std::hint::unreachable_unchecked;

use calsc_diagnostics::{DiagPossible, DiagResult};
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    builders::{build_array_gep, build_insert_value, build_store, build_struct_gep},
    module::Module,
    values::{
        BaseSSAValue, ValueType, int::SSAIntValue, ptr::SSAPointerValue, structs::SSAStructValue,
    },
};

use crate::{
    reads::lower_hir_readable_pointer, result::CalscinRemirResult, values::lower_hir_value,
    vars::lower_hir_variable_reference,
};

pub fn lower_hir_writable(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
    val: BaseSSAValue,
) -> DiagPossible {
    match node.kind {
        HIRNodeKind::VariableReference { .. } => {
            let r = lower_hir_variable_reference(node.clone(), local_ctx, module)?;

            r.write(module, val)
                .convert(node.start.clone(), node.end.clone())?;

            Ok(())
        }

        HIRNodeKind::FieldReference { .. } => {
            lower_hir_field_writable(node, local_ctx, module, val)
        }

        HIRNodeKind::PointerDereference(_) => {
            lower_hir_pointer_writable(node, local_ctx, module, val)
        }

        HIRNodeKind::IndexUsage { .. } => lower_hir_index_writable(node, local_ctx, module, val),

        _ => panic!(),
    }
}

pub fn lower_hir_writable_value(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    match node.kind {
        HIRNodeKind::VariableReference { .. } => {
            let var = lower_hir_variable_reference(node.clone(), local_ctx, module)?;

            return Ok(var.held_value.clone().unwrap());
        }

        _ => Ok(lower_hir_readable_pointer(node, local_ctx, module)?.into()),
    }
}

pub fn lower_hir_field_writable(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
    write_into: BaseSSAValue,
) -> DiagPossible {
    if let HIRNodeKind::FieldReference {
        val,
        field_ind,
        name: _,
    } = node.kind.clone()
    {
        let val = lower_hir_value(val, local_ctx, module)?;

        if let ValueType::Pointer(_) = &val.value_type {
            let val: SSAPointerValue = val
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;

            let ptr = build_struct_gep(module, val, field_ind)
                .convert(node.start.clone(), node.end.clone())?;

            build_store(module, ptr, write_into).convert(node.start.clone(), node.end.clone())
        } else {
            let val: SSAStructValue = val
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;

            build_insert_value(module, val, field_ind, write_into)
                .convert(node.start.clone(), node.end.clone())
        }
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_hir_pointer_writable(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
    write_into: BaseSSAValue,
) -> DiagPossible {
    if let HIRNodeKind::PointerDereference(inner) = node.kind.clone() {
        let inner = lower_hir_readable_pointer(inner, local_ctx, module)?;

        build_store(module, inner, write_into).convert(node.start.clone(), node.end.clone())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_hir_index_writable(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
    write_into: BaseSSAValue,
) -> DiagPossible {
    if let HIRNodeKind::IndexUsage {
        val,
        index,
        output_type: _,
    } = node.kind.clone()
    {
        let val = lower_hir_readable_pointer(val, local_ctx, module)?;

        let index = lower_hir_value(index, local_ctx, module)?;
        let index: SSAIntValue = index
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let ptr =
            build_array_gep(module, val, index).convert(node.start.clone(), node.end.clone())?;

        build_store(module, ptr, write_into).convert(node.start.clone(), node.end.clone())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

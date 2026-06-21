use calsc_diagnostics::{
    DiagPossible, DiagResult,
    diags::errors::{
        build_expected_mutable, build_expected_mutable_reference, build_internal_hir_node_leaked,
    },
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
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
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    val: BaseSSAValue,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node);

    match &node_ref.kind {
        HIRNodeKind::VariableReference { .. } => {
            let r = lower_hir_variable_reference(node.clone(), local_ctx, module, ctx)?;

            r.write(module, val)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            Ok(())
        }

        HIRNodeKind::FieldReference { .. } => {
            lower_hir_field_writable(node, local_ctx, module, val, ctx)
        }

        HIRNodeKind::PointerDereference(_) => {
            lower_hir_pointer_writable(node, local_ctx, module, val, ctx)
        }

        HIRNodeKind::IndexUsage { .. } => {
            lower_hir_index_writable(node, local_ctx, module, val, ctx)
        }

        _ => panic!(),
    }
}

pub fn lower_hir_writable_value(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    ctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = ctx.nodes.get(&node);

    match node_ref.kind {
        HIRNodeKind::VariableReference { .. } => {
            let var = lower_hir_variable_reference(node.clone(), local_ctx, module, ctx)?;

            return Ok(var.held_value.clone().unwrap());
        }

        _ => Ok(lower_hir_readable_pointer(node, local_ctx, module, ctx)?.into()),
    }
}

pub fn lower_hir_field_writable(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    write_into: BaseSSAValue,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::FieldReference {
        val,
        field_ind,
        name: _,
    } = node_ref.kind.clone()
    {
        let val = lower_hir_value(val, local_ctx, module, ctx)?;

        if let ValueType::Pointer(_) = &val.value_type {
            let val: SSAPointerValue = val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let ptr = build_struct_gep(module, val, field_ind)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            build_store(module, ptr, write_into)
                .convert(node_ref.start.clone(), node_ref.end.clone())
        } else {
            let val: SSAStructValue = val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            build_insert_value(module, val, field_ind, write_into)
                .convert(node_ref.start.clone(), node_ref.end.clone())
        }
    } else {
        return Err(build_internal_hir_node_leaked(&*node_ref, &*node_ref).into());
    }
}

pub fn lower_hir_pointer_writable(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    write_into: BaseSSAValue,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::PointerDereference(inner) = node_ref.kind.clone() {
        //if !node_ref.represents_mutable_variable(
        // ctx,
        // Some(local_ctx.local_key.clone()),
        //node_ref,
        //)? {
        //return Err(build_expected_mutable(node_ref).into());
        //}
        //

        let inner_ref = ctx.nodes.get(&inner);
        let ty = inner_ref.get_type(Some(local_ctx.local_key.clone()), ctx, None)?;

        if !ty.is_type_mutable_compatible() {
            return Err(build_expected_mutable_reference(&ty, node_ref).into());
        }

        let inner = lower_hir_readable_pointer(inner, local_ctx, module, ctx)?;

        build_store(module, inner, write_into).convert(node_ref.start.clone(), node_ref.end.clone())
    } else {
        return Err(build_internal_hir_node_leaked(&*node_ref, &*node_ref).into());
    }
}

pub fn lower_hir_index_writable(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    write_into: BaseSSAValue,
    ctx: &HIRContext,
) -> DiagPossible {
    let node_ref = ctx.nodes.get(&node);

    if let HIRNodeKind::IndexUsage {
        val,
        index,
        output_type: _,
    } = node_ref.kind.clone()
    {
        let val = lower_hir_readable_pointer(val, local_ctx, module, ctx)?;

        let index = lower_hir_value(index, local_ctx, module, ctx)?;
        let index: SSAIntValue = index
            .try_into()
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        let ptr = build_array_gep(module, val, index)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        build_store(module, ptr, write_into).convert(node_ref.start.clone(), node_ref.end.clone())
    } else {
        return Err(build_internal_hir_node_leaked(&*node_ref, &*node_ref).into());
    }
}

use std::hint::unreachable_unchecked;

use calsc_diagnostics::DiagPossible;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    builders::{build_insert_value, build_store, build_struct_gep},
    module::Module,
    values::{ptr::SSAPointerValue, structs::SSAStructValue},
};

use crate::{
    result::CalscinRemirResult, values::lower_hir_value, vars::lower_hir_variable_reference,
};

pub fn lower_hir_pointer_deref_assign(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::PointerDerefAssign { pointer, value } = node.kind.clone() {
        let pointer = lower_hir_value(pointer, local_ctx, module)?;
        let pointer: SSAPointerValue = pointer
            .try_into()
            .convert(node.start.clone(), node.end.clone())?;

        let value = lower_hir_value(value, local_ctx, module)?;

        build_store(module, pointer, value).convert(node.start.clone(), node.end.clone())?;

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_hir_struct_field_assign(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::StructFieldAssign {
        struct_val,
        field: _,
        field_ind,
        value,
    } = node.kind.clone()
    {
        let value = lower_hir_value(value, local_ctx, module)?;
        let r = lower_hir_variable_reference(struct_val, local_ctx, module)?;
        let held_value = r.held_value.clone().unwrap();

        // TODO: add struct helper in Remir

        if r.write_as_pointer {
            // TODO: support atomicity when r.write_as_pointer is true

            let held_value: SSAPointerValue = held_value
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;

            let ptr = build_struct_gep(module, held_value, field_ind)
                .convert(node.start.clone(), node.end.clone())?;

            build_store(module, ptr, value).convert(node.start.clone(), node.end.clone())?;
        } else {
            let held_value: SSAStructValue = held_value
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;

            build_insert_value(module, held_value, field_ind, value)
                .convert(node.start.clone(), node.end.clone())?;
        }

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

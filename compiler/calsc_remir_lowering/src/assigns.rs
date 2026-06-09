use std::hint::unreachable_unchecked;

use calsc_diagnostics::DiagPossible;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{builders::build_store, module::Module, values::ptr::SSAPointerValue};

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
        field,
        value,
    } = node.kind.clone()
    {
        let r = lower_hir_variable_reference(struct_val, local_ctx, module)?;
        let held_value = r.held_value.clone().unwrap();

        // TODO: add struct helper in Remir
        // TODO: support atomicity when r.write_as_pointer is true

        if r.write_as_pointer {
            let held_value: SSAPointerValue = held_value
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;

			let ptr = buildstruct

        }

        todo!()
    } else {
        unsafe { unreachable_unchecked() }
    }
}

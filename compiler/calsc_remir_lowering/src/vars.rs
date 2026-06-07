use std::{hint::unreachable_unchecked, mem::transmute};

use calsc_diagnostics::{DiagPossible, DiagResult};
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{block::vars::BlockVariable, module::Module, values::BaseSSAValue};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

pub fn lower_hir_variable_reference<'a>(
    node: HIRArenaReference,
    _ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<&'a mut BlockVariable> {
    if let HIRNodeKind::VariableReference {
        name,
        variable_index: _,
    } = node.kind.clone()
    {
        let block = &mut module.blocks[module.pos_block.as_ref().unwrap().id];
        let variable = block.variables.get_mut(&*name).unwrap();

        unsafe {
            Ok(transmute::<&mut BlockVariable, &'a mut BlockVariable>(
                variable,
            ))
        }
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_hir_variable_reference_val(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    let var = lower_hir_variable_reference(node.clone(), ctx, module)?;

    var.read(module)
        .convert(node.start.clone(), node.end.clone())
}

pub fn lower_hir_variable_assign(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::Assignment { variable, value } = node.kind.clone() {
        let variable = lower_hir_variable_reference(variable, ctx, module)?;
        let value = lower_hir_value(value, ctx, module)?;

        variable
            .write(module, value)
            .convert(node.start.clone(), node.end.clone())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

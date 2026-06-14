use std::mem::transmute;

use calsc_diagnostics::{DiagPossible, DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    block::vars::BlockVariable,
    builders::{build_alloca, build_const_int},
    module::Module,
    values::BaseSSAValue,
};

use crate::{
    result::CalscinRemirResult, types::lower_type, values::lower_hir_value,
    writes::lower_hir_writable,
};

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
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
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
        let value = lower_hir_value(value, ctx, module)?;

        lower_hir_writable(variable, ctx, module, value)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

pub fn lower_hir_variable_declaration(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::VariableDeclaration {
        mutable,
        var_type,
        value,
        name,
        variable_index,
    } = node.kind.clone()
    {
        let mut variable: BlockVariable;
        let is_array = var_type.is_array();

        let var_type = lower_type(var_type).unwrap();

        if mutable || ctx.variables[variable_index].reference_count > 0 || is_array {
            // Uses a stack variable for mutable variables.
            // TODO: allow to customize this in the future

            let size = build_const_int(module, 0, 32, false)
                .convert(node.start.clone(), node.end.clone())?;

            let ptr = build_alloca(module, size, Some(var_type.clone()))
                .convert(node.start.clone(), node.end.clone())?;

            variable = BlockVariable::new_pointer(String::clone(&name), ptr);
        } else {
            // Uses a SSA variable for immutable variables.
            // TODO: allow to customize this in the future
            variable = BlockVariable::new_ssa(String::clone(&name), None);
        }

        if value.is_some() {
            let value = value.unwrap();
            let value = lower_hir_value(value, ctx, module)?;

            variable
                .write(module, value)
                .convert(node.start.clone(), node.end.clone())?;
        }

        let block = &mut module.blocks[module.pos_block.as_ref().unwrap().id];
        block.variables.insert(String::clone(&name), variable);

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

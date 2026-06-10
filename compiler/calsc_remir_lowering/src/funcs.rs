use std::hint::unreachable_unchecked;

use calsc_diagnostics::{
    DiagPossible, DiagResult, diags::errors::build_cannot_find_element_no_closest,
};
use calsc_hir::{
    HIRContext, globalctx::key::GlobalContextKey, localctx::LocalContext, nodes::HIRNodeKind,
    refs::HIRArenaReference,
};
use calsc_typing::tree::Type;
use remir::{
    block::vars::BlockVariable,
    builders::{build_argument_grab, build_call, build_ret},
    module::Module,
    values::BaseSSAValue,
    writer::InstructionWriter,
};

use crate::{
    body::lower_hir_body, result::CalscinRemirResult, types::lower_type, values::lower_hir_value,
};

pub fn lower_hir_function_call(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<Option<BaseSSAValue>> {
    if let HIRNodeKind::FunctionCall { func, arguments } = node.kind.clone() {
        let name = format!("{}", func);
        let reference_label = match module.get_function_by_name(name.clone()) {
            Some(v) => v,
            None => return Err(build_cannot_find_element_no_closest(&name, &*node).into()),
        };

        let mut lowered_arguments = vec![];

        for argument in arguments {
            let v = lower_hir_value(argument, local_ctx, module)?;

            lowered_arguments.push(v);
        }

        let val = build_call(
            module,
            reference_label,
            lowered_arguments,
            false,
            false,
            false,
        )
        .convert(node.start.clone(), node.end.clone())?;

        Ok(val)
    } else {
        panic!()
    }
}

pub fn lower_hir_function_decl_none(
    key: GlobalContextKey,
    arguments: Vec<Type>,
    return_type: Option<Type>,
    module: &mut Module,
) -> DiagPossible {
    let mut mir_arguments = vec![];
    let mir_return_type;

    for argument in arguments {
        mir_arguments.push(lower_type(argument)?);
    }

    if return_type.is_some() {
        mir_return_type = Some(lower_type(return_type.unwrap())?);
    } else {
        mir_return_type = None;
    }

    let _ = module.create_function(format!("{}", key), mir_arguments, mir_return_type);

    Ok(())
}

pub fn lower_hir_function_decl(
    node: HIRArenaReference,
    context: &HIRContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::FunctionDeclaration {
        key,
        arguments,
        body,
        return_type: _,
    } = node.kind.clone()
    {
        let local_context = context
            .scope
            .get_entry(key.clone(), &*node)?
            .as_function(&*node)?
            .local_context
            .clone()
            .unwrap();

        let function = module.get_function_by_name(format!("{}", key)).unwrap();
        module.move_function(function.clone());

        let entry_block = module
            .create_block(format!("{}::entry", key))
            .convert(node.start.clone(), node.end.clone())?;

        module.move_end(entry_block.clone(), function);

        // Load arguments
        {
            let mut argument_ind = 0;
            for argument in &arguments {
                let val = build_argument_grab(module, argument_ind)
                    .convert(node.start.clone(), node.end.clone())?;

                let argument_variable =
                    BlockVariable::new_ssa(String::clone(&argument.1), Some(val));

                module.blocks[entry_block.id].append_variable(argument_variable);

                argument_ind += 1;
            }
        }

        lower_hir_body(body, &local_context, module)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_hir_function_return(
    node: HIRArenaReference,
    local_ctx: &LocalContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::ReturnStatement { val } = node.kind.clone() {
        let mir_val;

        if val.is_some() {
            mir_val = Some(lower_hir_value(val.unwrap(), local_ctx, module)?);
        } else {
            mir_val = None;
        }

        build_ret(module, mir_val);

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

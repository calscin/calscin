use calsc_diagnostics::{
    DiagPossible, DiagResult,
    diags::errors::{build_cannot_find_element_no_closest, build_internal_hir_node_leaked},
};
use calsc_hir::{
    HIRContext, globalctx::key::GlobalContextKey, localctx::LocalContext, nodes::HIRNodeKind,
};

use calsc_typing_v2::{ctx::TypeCtx, types::TypeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    block::vars::BlockVariable,
    builders::{build_argument_grab, build_call, build_const_int, build_ret},
    module::Module,
    values::{BaseSSAValue, ValueType},
    writer::InstructionWriter,
};

use crate::{
    body::lower_hir_body, result::CalscinRemirResult, types::lower_type, values::lower_hir_value,
};

pub fn lower_hir_function_call(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    hirctx: &mut HIRContext,
) -> DiagResult<Option<BaseSSAValue>> {
    let node_ref = hirctx.nodes.get(&node).clone();

    if let HIRNodeKind::FunctionCall { func, arguments } = node_ref.kind.clone() {
        let name = format!("{}", func);
        let reference_label = match module.get_function_by_name(name.clone()) {
            Some(v) => v,
            None => return Err(build_cannot_find_element_no_closest(&name, &node_ref).into()),
        };

        let mut lowered_arguments = vec![];

        for argument in arguments {
            let v = lower_hir_value(argument, local_ctx, module, hirctx)?;

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
        .convert(node_ref.start.clone(), node_ref.end.clone())?;

        Ok(val)
    } else {
        return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into());
    }
}

pub fn lower_hir_function_decl_none(
    key: GlobalContextKey,
    arguments: Vec<TypeKind>,
    return_type: TypeKind,
    is_main_function: bool,
    module: &mut Module,
    ctx: &TypeCtx,
) -> DiagPossible {
    let mut mir_arguments = vec![];
    let mut mir_return_type = lower_type(return_type, ctx)?;

    for argument in arguments {
        mir_arguments.push(lower_type(argument, ctx)?);
    }

    if is_main_function {
        mir_arguments = vec![
            ValueType::Int(true, 64),
            ValueType::new_pointer(ValueType::new_any_pointer()),
        ];

        mir_return_type = ValueType::Int(true, 32);
    }

    let _ = module.create_function(format!("{}", key), mir_arguments, mir_return_type);

    Ok(())
}

pub fn lower_hir_function_decl(
    node: ArenaHandle,
    context: &mut HIRContext,
    module: &mut Module,
) -> DiagPossible {
    let node_ref = context.nodes.get(&node);

    if let HIRNodeKind::FunctionDeclaration {
        key,
        arguments,
        body,
        return_type: _,
        append_terminator,
    } = node_ref.kind.clone()
    {
        let local_context = context
            .scope
            .get_entry_no_visibility(key.clone(), node_ref)?
            .as_function(node_ref)?
            .local_context
            .clone()
            .unwrap();

        let function = module.get_function_by_name(format!("{}", key)).unwrap();
        module.move_function(function.clone());

        let entry_block = module
            .create_block(format!("{}::entry", key))
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        module.move_end(entry_block.clone(), function);

        // Load arguments
        {
            let mut argument_ind = 0;
            for argument in &arguments {
                let val = build_argument_grab(module, argument_ind)
                    .convert(node_ref.start.clone(), node_ref.end.clone())?;

                let argument_variable =
                    BlockVariable::new_ssa(String::clone(&argument.1), Some(val));

                module.blocks[entry_block.id].append_variable(argument_variable);

                argument_ind += 1;
            }
        }

        lower_hir_body(body, &local_context, module, context)?;

        if append_terminator {
            let return_type = build_const_int(module, 0, 32, true).unwrap();

            build_ret(module, Some(return_type.into()));
        }

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node_ref).into());
    }
}

pub fn lower_hir_function_return(
    node: ArenaHandle,
    local_ctx: &LocalContext,
    module: &mut Module,
    hirctx: &mut HIRContext,
) -> DiagPossible {
    let node_ref = hirctx.nodes.get(&node);

    if let HIRNodeKind::ReturnStatement { val } = node_ref.kind.clone() {
        let mir_val;

        if val.is_some() {
            mir_val = Some(lower_hir_value(val.unwrap(), local_ctx, module, hirctx)?);
        } else {
            mir_val = None;
        }

        build_ret(module, mir_val);

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(node_ref, node_ref).into());
    }
}

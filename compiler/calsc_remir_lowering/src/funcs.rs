use std::hint::unreachable_unchecked;

use calsc_diagnostics::{
    DiagPossible, DiagResult, diags::errors::build_cannot_find_element_no_closest,
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{
    builders::build_call, module::Module, values::BaseSSAValue, writer::InstructionWriter,
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
            lowered_arguments.push(lower_hir_value(argument, local_ctx, module)?)
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

pub fn lower_hir_function_decl(
    node: HIRArenaReference,
    context: &HIRContext,
    module: &mut Module,
) -> DiagPossible {
    if let HIRNodeKind::FunctionDeclaration {
        key,
        arguments,
        body,
        return_type,
    } = node.kind.clone()
    {
        let local_context = context
            .scope
            .get_entry(key.clone(), &*node)?
            .as_function(&*node)?
            .local_context
            .clone()
            .unwrap();

        let mut mir_arguments = vec![];
        let mir_return_type;

        for argument in arguments {
            mir_arguments.push(lower_type(argument.0)?);
        }

        if return_type.is_some() {
            mir_return_type = Some(lower_type(return_type.unwrap())?);
        } else {
            mir_return_type = None;
        }

        let function = module.create_function(format!("{}", key), mir_arguments, mir_return_type);
        module.move_function(function.clone());

        let entry_block = module
            .create_block(format!("{}::entry", key))
            .convert(node.start.clone(), node.end.clone())?;

        module.move_end(entry_block, function);

        lower_hir_body(body, &local_context, module)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

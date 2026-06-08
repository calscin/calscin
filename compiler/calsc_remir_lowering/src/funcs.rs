use calsc_diagnostics::{DiagResult, diags::errors::build_cannot_find_element_no_closest};
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{builders::build_call, module::Module, values::BaseSSAValue};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

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

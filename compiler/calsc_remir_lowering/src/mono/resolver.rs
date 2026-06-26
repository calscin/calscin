use std::hash::{DefaultHasher, Hash, Hasher};

use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_cannot_find_element_no_closest, build_internal_hir_node_leaked},
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_typing::hash::HashedTypeKind;
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{builders::build_call, module::Module, values::BaseSSAValue};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

pub fn lower_hir_typed_function_call(
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &mut HIRContext,
) -> DiagResult<Option<BaseSSAValue>> {
    let node_ref = hirctx.nodes.get(&node).clone();

    if let HIRNodeKind::TypedParamFunctionCall {
        func,
        arguments,
        type_parameters,
    } = node_ref.kind.clone()
    {
        let group = hirctx.type_ctx.type_params.start_param_group();

        for (name, param) in type_parameters.clone() {
            hirctx.type_ctx.type_params.append_resolved(name, param);
        }

        let type_params_function = hirctx
            .scope
            .get_entry_no_visibility(func.clone(), &node_ref)?
            .as_function(&node_ref)?
            .type_parameters
            .clone();

        let mut name = format!("{}", func);

        // Build the suffix hash
        let mut suffix = DefaultHasher::new();

        for param in type_params_function {
            let ty = HashedTypeKind::new(type_parameters[&param.1].clone(), &hirctx.type_ctx);

            ty.hash(&mut suffix);
        }

        let suffix = suffix.finish();

        name += &format!("#__{}__", suffix);

        let reference_label = match module.get_function_by_name(name.to_string()) {
            Some(v) => v,
            None => return Err(build_cannot_find_element_no_closest(&name, &node_ref).into()),
        };

        let mut lowered_arguments = vec![];

        for argument in arguments {
            let v = lower_hir_value(argument, ctx, module, hirctx)?;

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

        hirctx.type_ctx.type_params.end_group(group);

        Ok(val)
    } else {
        return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into());
    }
}

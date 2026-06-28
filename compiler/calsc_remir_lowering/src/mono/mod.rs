use std::hash::{DefaultHasher, Hash, Hasher};

use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{
    BUILD_CACHE, HIRContext,
    funcs::HIRFunction,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_typing::{hash::HashedTypeKind, types::TypeKind};
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};
use remir::{
    block::vars::BlockVariable,
    builders::{build_argument_grab, build_const_int, build_ret},
    module::Module,
    writer::InstructionWriter,
};

use crate::{body::lower_hir_body, result::CalscinRemirResult, types::lower_type};

pub mod resolver;

pub struct Monomorphizer;

impl Monomorphizer {
    pub fn monomorph_function_definitions(
        module: &mut Module,
        func: &HIRFunction,
        context: &mut HIRContext,
    ) -> DiagPossible {
        let mut module_path = func.name.module_path.clone();
        module_path.append_single_bit(func.name.name.clone());

        let entry = BUILD_CACHE.with_borrow(|cache| cache.used_type_params[&module_path].clone());

        for combination in entry {
            let mut suffix = DefaultHasher::new();
            let mut name = format!("{}", func.name);

            let group = context.type_ctx.type_params.start_param_group();

            // Add combinations to the type ctx
            for (ind, elem) in combination.iter().enumerate() {
                context
                    .type_ctx
                    .type_params
                    .append_resolved(func.type_parameters[ind].1.clone(), elem.kind.clone());
            }

            for elem in combination {
                elem.hash(&mut suffix);
            }

            let suffix = suffix.finish();

            name += &format!("#__{}__", suffix);

            let mut mir_arguments = vec![];
            let mir_return_type = lower_type(func.return_type.clone(), &context.type_ctx)?;

            for argument in &func.arguments {
                mir_arguments.push(lower_type(argument.1.clone(), &context.type_ctx)?);
            }

            let _ = module.create_function(name, mir_arguments, mir_return_type);

            context.type_ctx.type_params.end_group(group);
        }

        Ok(())
    }

    fn monomorch_function_decl(
        name: String,
        key: GlobalContextKey,
        node_ref: &HIRNode,
        arguments: Vec<(TypeKind, HashedString)>,
        body: Vec<ArenaHandle>,
        append_terminator: bool,
        combination: Vec<HashedTypeKind>,
        context: &mut HIRContext,
        module: &mut Module,
    ) -> DiagPossible {
        let group = context.type_ctx.type_params.start_param_group();

        let type_parameters = context
            .scope
            .get_entry_no_visibility(key.clone(), node_ref)?
            .as_function(node_ref)?
            .type_parameters
            .clone();

        let local_context = context
            .scope
            .get_entry_no_visibility(key.clone(), node_ref)?
            .as_function(node_ref)?
            .local_context
            .clone()
            .unwrap();

        let function = module.get_function_by_name(name.clone()).unwrap();
        module.move_function(function.clone());

        for (ind, elem) in combination.iter().enumerate() {
            context
                .type_ctx
                .type_params
                .append_resolved(type_parameters[ind].1.clone(), elem.kind.clone());
        }

        let entry_block = module
            .create_block(format!("{}::entry", name.to_string()))
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

        context.type_ctx.type_params.end_group(group);

        Ok(())
    }

    pub fn monomorph_function_declarations(
        node: ArenaHandle,
        context: &mut HIRContext,
        module: &mut Module,
    ) -> DiagPossible {
        let node_ref = context.nodes.get(&node).clone();

        if let HIRNodeKind::FunctionDeclaration {
            key,
            arguments,
            body,
            return_type: _,
            append_terminator,
        } = node_ref.kind.clone()
        {
            let mut module_path = key.module_path.clone();
            module_path.append_single_bit(key.name.clone());

            let entry =
                BUILD_CACHE.with_borrow(|cache| cache.used_type_params[&module_path].clone());

            for combination in entry {
                let mut suffix = DefaultHasher::new();
                let mut name = format!("{}", key.clone());

                for elem in combination.clone() {
                    elem.hash(&mut suffix);
                }

                let suffix = suffix.finish();

                name += &format!("#__{}__", suffix);

                Monomorphizer::monomorch_function_decl(
                    name,
                    key.clone(),
                    &node_ref,
                    arguments.clone(),
                    body.clone(),
                    append_terminator,
                    combination,
                    context,
                    module,
                )?;
            }

            Ok(())
        } else {
            return Err(build_internal_hir_node_leaked(&node, &node_ref).into());
        }
    }
}

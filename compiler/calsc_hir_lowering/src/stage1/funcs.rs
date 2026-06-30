use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{
        build_internal_hir_node_leaked, build_restricted_arument_type,
        build_restricted_return_type, build_restricted_type_parameters,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
    localctx::LocalContext,
};
use calsc_typing::types::TypeKind;

use crate::{convert_visibility, stage1::types::lower_ast_type};

pub fn lower_ast_function_decl_first_stage(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body: _,
        visibility,
        type_parameters,
    } = node.kind.clone()
    {
        let group = ctx.type_ctx.type_params.start_param_group();

        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());
        let mut owned_type_params = vec![];

        for type_parameter in type_parameters {
            let id = ctx
                .type_ctx
                .type_params
                .append_type_param(type_parameter, &node)?;

            owned_type_params.push(id);
        }

        let mut key =
            GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

        let is_main_function =
            key.name == "main".into() && key.module_path == file_ctx.current_module;

        if is_main_function {
            key = GlobalContextKey::new("main".into());
        }

        let mut args = vec![];
        let ret_type = lower_ast_type(return_type, &node, file_ctx, ctx)?;

        let mut local_ctx = LocalContext::new(
            name.clone(),
            key.clone(),
            ret_type.clone(),
            is_main_function,
        );

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, file_ctx, ctx)?;

            local_ctx.introduce_variable(argument.1.clone(), ty.clone(), false, true, &node)?;
            args.push((argument.1, ty));
        }

        if is_main_function {
            if !args.is_empty() {
                return Err(build_restricted_arument_type(&vec!["void".to_string()], &node).into());
            }

            if ret_type != TypeKind::Void {
                return Err(build_restricted_return_type(&"void".to_string(), &node).into());
            }

            if !owned_type_params.is_empty() {
                return Err(
                    build_restricted_type_parameters::<String, ASTNode>(&vec![], &node).into(),
                );
            }
        }

        let mut func =
            HIRFunction::new_stage_1(key.clone(), local_ctx, ret_type, args, is_main_function);

        func.type_parameters = owned_type_params;

        let _ = ctx
            .scope
            .append(key, GlobalContextValue::Function(func), visibility, &node)?;

        ctx.type_ctx.type_params.end_group(group);

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_extern_function(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagPossible {
    if let ASTNodeKind::ExternFunctionDeclaration {
        name,
        arguments,
        return_type,
        triple_dot_position,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        let key = GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

        let mut args = vec![];
        let ret_type = lower_ast_type(return_type, &node, file_ctx, ctx)?;

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, file_ctx, ctx)?;

            args.push((argument.1, ty));
        }

        let func = HIRFunction::new_extern(key.clone(), ret_type, args, triple_dot_position, false);

        let _ = ctx
            .scope
            .append(key, GlobalContextValue::Function(func), visibility, &node)?;

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

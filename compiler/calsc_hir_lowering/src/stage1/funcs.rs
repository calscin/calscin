use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{
        build_internal_hir_node_leaked, build_restricted_arument_type, build_restricted_return_type,
    },
};
use calsc_hir::{
    HIR_CONTEXT,
    file::HIRFileContext,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
    localctx::LocalContext,
};
use calsc_typing::{
    base::BaseType,
    func::{MutableDeclBlockAffectedType, TypedFunction},
    tree::Type,
};

use crate::stage1::types::lower_ast_type;

pub fn lower_ast_function_decl_first_stage(
    node: ASTNode,
    target: Option<BaseType>,
    file_ctx: &mut HIRFileContext,
) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body: _,
    } = node.kind.clone()
    {
        let mut key =
            GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

        if target.is_some() {
            key = key.associated_type(target.clone().unwrap());
        }

        let is_main_function = key.name == "main".into() && key.module_path.path.len() == 0;

        if is_main_function {
            key = GlobalContextKey::new("main".into());
        }

        let mut args = vec![];
        let ret_type = lower_ast_type(return_type, &node, target.clone(), file_ctx)?;

        let mut local_ctx = LocalContext::new(
            name.clone(),
            key.clone(),
            ret_type.clone(),
            is_main_function,
        );

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, target.clone(), file_ctx)?;

            local_ctx.introduce_variable(argument.1.clone(), ty.clone(), true, &node)?;
            args.push((argument.1, ty));
        }

        if is_main_function {
            if !args.is_empty() {
                return Err(build_restricted_arument_type(&vec!["void".to_string()], &node).into());
            }

            if ret_type != Type::Void {
                return Err(build_restricted_return_type(&"void".to_string(), &node).into());
            }
        }

        if target.is_some() {
            let k = GlobalContextKey::new(target.clone().unwrap().kind.get_name());

            let mut argument_types = vec![];

            for arg in &args {
                argument_types.push(arg.1.clone());
            }

            let typed = TypedFunction::new((*name).clone(), argument_types, ret_type.clone());

            HIR_CONTEXT.with(|f| {
                f.borrow_mut().scope.mutate_entry(
                    k,
                    |e| e.mutate_type(|typ| typ.add_function(name, typed, &node), &node),
                    &node,
                )
            })???;
        }

        let func = HIRFunction::new_stage_1(
            key.clone(),
            local_ctx,
            target,
            ret_type,
            args,
            is_main_function,
        );

        let _ = HIR_CONTEXT.with(|f| {
            f.borrow_mut()
                .scope
                .append(key, GlobalContextValue::Function(func), &node)
        })?;

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_extern_function(node: ASTNode, file_ctx: &mut HIRFileContext) -> DiagPossible {
    if let ASTNodeKind::ExternFunctionDeclaration {
        name,
        arguments,
        return_type,
        triple_dot_position,
    } = node.kind.clone()
    {
        let key = GlobalContextKey::new(name.clone());

        let mut args = vec![];
        let ret_type = lower_ast_type(return_type, &node, None, file_ctx)?;

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, None, file_ctx)?;

            args.push((argument.1, ty));
        }

        let func = HIRFunction::new_extern(
            key.clone(),
            None,
            ret_type,
            args,
            triple_dot_position,
            false,
        );

        let _ = HIR_CONTEXT.with(|f| {
            f.borrow_mut()
                .scope
                .append(key, GlobalContextValue::Function(func), &node)
        })?;

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

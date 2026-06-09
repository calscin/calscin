use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagPossible;
use calsc_hir::{
    HIR_CONTEXT,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
    localctx::LocalContext,
};
use calsc_typing::{
    base::BaseType,
    func::{MutableDeclBlockAffectedType, TypedFunction},
};

use crate::stage1::types::lower_ast_type;

pub fn lower_ast_function_decl_first_stage(
    node: ASTNode,
    target: Option<BaseType>,
) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body: _,
    } = node.kind.clone()
    {
        let mut key = GlobalContextKey::new(name.clone());

        if target.is_some() {
            key = GlobalContextKey::new_typed(name.clone(), target.clone().unwrap());
        }

        let mut args = vec![];
        let mut ret_type = None;

        if let Some(v) = return_type {
            ret_type = Some(lower_ast_type(v, &node, target.clone())?);
        }

        let mut local_ctx = LocalContext::new(name.clone(), key.clone(), ret_type.clone());

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, target.clone())?;

            local_ctx.introduce_variable(argument.1.clone(), ty.clone(), true, &node)?;
            args.push((argument.1, ty));
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

        let func = HIRFunction::new_stage_1(key.clone(), local_ctx, target, ret_type, args);

        let _ = HIR_CONTEXT.with(|f| {
            f.borrow_mut()
                .scope
                .append(key, GlobalContextValue::Function(func), &node)
        })?;

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_extern_function(node: ASTNode) -> DiagPossible {
    if let ASTNodeKind::ExternFunctionDeclaration {
        name,
        arguments,
        return_type,
        triple_dot_position,
    } = node.kind.clone()
    {
        let key = GlobalContextKey::new(name.clone());

        let mut args = vec![];
        let mut ret_type = None;

        if let Some(v) = return_type {
            ret_type = Some(lower_ast_type(v, &node, None)?);
        }

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, None)?;

            args.push((argument.1, ty));
        }

        let func = HIRFunction::new_extern(key.clone(), None, ret_type, args, triple_dot_position);

        let _ = HIR_CONTEXT.with(|f| {
            f.borrow_mut()
                .scope
                .append(key, GlobalContextValue::Function(func), &node)
        })?;

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

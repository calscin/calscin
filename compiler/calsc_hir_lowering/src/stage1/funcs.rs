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
            ret_type = Some(lower_ast_type(v, &node, target.clone())?); // TODO: pass struct type in order to propagate struct decl block type parameter further
        }

        let mut local_ctx = LocalContext::new(name.clone(), ret_type.clone());

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, target.clone())?;

            local_ctx.introduce_variable(argument.1.clone(), ty.clone(), true, &node)?;
            args.push((argument.1, ty)); // TODO: pass struct type in order to propagate struct decl block type parameter further
        }

        if target.is_some() {
            let k = GlobalContextKey::new(target.clone().unwrap().kind.get_name());

            let mut argument_types = vec![];

            for arg in &args {
                argument_types.push(arg.1.clone());
            }

            let typed = TypedFunction::new((*name).clone(), argument_types, ret_type.clone());

            HIR_CONTEXT.with_borrow_mut(|f| {
                f.scope.mutate_entry(
                    k,
                    |e| e.mutate_type(|typ| typ.add_function(name, typed, &node), &node),
                    &node,
                )
            })?;
        }

        let func = HIRFunction::new_stage_1(key.clone(), local_ctx, target, ret_type, args); // TODO: pass struct type in order to propagate struct decl block type parameter further

        let _ = HIR_CONTEXT.with_borrow_mut(|f| {
            f.scope
                .append(key, GlobalContextValue::Function(func), &node)
        })?;

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

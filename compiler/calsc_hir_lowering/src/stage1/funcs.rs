use std::{clone, hint::unreachable_unchecked};

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagPossible;
use calsc_hir::{
    HIR_CONTEXT,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
    localctx::LocalContext,
};
use calsc_utils::hash::HashedString;

use crate::stage1::types::lower_ast_type;

pub fn lower_ast_function_decl_first_stage(node: ASTNode) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body,
    } = node.kind
    {
        let key = GlobalContextKey::new(name.clone());

        let mut args = vec![];
        let mut ret_type = None;

        if let Some(v) = return_type {
            ret_type = Some(lower_ast_type(v, &node, None)?); // TODO: pass struct type in order to propagate struct decl block type parameter further
        }

        let mut local_ctx = LocalContext::new(name.clone(), ret_type.clone());

        for argument in arguments {
            let ty = lower_ast_type(argument.0, &node, None)?;

            local_ctx.introduce_variable(argument.1.clone(), ty.clone(), true, &node);
            args.push((argument.1, ty)); // TODO: pass struct type in order to propagate struct decl block type parameter further
        }

        let func = HIRFunction::new_stage_1(key.clone(), local_ctx, None, ret_type, args); // TODO: pass struct type in order to propagate struct decl block type parameter further

        let _ = HIR_CONTEXT.with_borrow_mut(|f| {
            f.scope
                .append(key, GlobalContextValue::Function(func), &node)
        })?;

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

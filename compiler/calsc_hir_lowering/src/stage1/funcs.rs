use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagPossible;
use calsc_hir::globalctx::key::GlobalContextKey;
use calsc_utils::hash::HashedString;

use crate::stage1::types::lower_ast_type;

pub fn lower_ast_function_decl_first_stage(node: ASTNode) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
		return_type
        body,
    } = node.kind
    {
        let key = GlobalContextKey::new(name);

        let mut args = vec![];

        for argument in arguments {
            args.push((lower_ast_type(argument.0, &node, None)?, argument.1)); // TODO: pass struct type in order to propagate struct decl block type parameter further
        }

		let mut ret_type = None;

		if let Some(v) = return_type {
			ret_type = Some(lower_ast_type(v, &node, None)?); // TODO: pass struct type in order to propagate struct decl block type parameter further
		}
    } else {
        unsafe { unreachable_unchecked() }
    }
}

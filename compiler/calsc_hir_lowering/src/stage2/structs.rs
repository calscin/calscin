use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagPossible;

use crate::{stage1::types::lower_simple_ast_type, stage2::funcs::lower_ast_function_decl};

pub fn lower_ast_struct_decl(node: ASTNode) -> DiagPossible {
    if let ASTNodeKind::StructDeclBlock { target, functions } = node.kind.clone() {
        let target = lower_simple_ast_type(target, &node, None)?;

        for func in functions {
            let _ = lower_ast_function_decl(ASTNode::clone(&func), Some(target.clone()))?;
        }

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

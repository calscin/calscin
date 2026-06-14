use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::file::HIRFileContext;

use crate::{stage1::types::lower_simple_ast_type, stage2::funcs::lower_ast_function_decl};

pub fn lower_ast_struct_decl(node: ASTNode, file_ctx: &mut HIRFileContext) -> DiagPossible {
    if let ASTNodeKind::StructDeclBlock { target, functions } = node.kind.clone() {
        let target = lower_simple_ast_type(target, &node, None)?;

        for func in functions {
            let _ = lower_ast_function_decl(ASTNode::clone(&func), Some(target.clone()), file_ctx)?;
        }

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

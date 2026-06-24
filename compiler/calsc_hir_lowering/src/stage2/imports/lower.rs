use calsc_ast::{
    ASTContext,
    imports::ImportKind,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{build_expected_entry_type, build_internal_hir_node_leaked},
};
use calsc_hir::{HIRContext, file::HIRFileContext};
use calsc_modules::tree::{ModuleTree, entry::ModuleTreeEntry};
use calsc_state::GLOBAL_STATE;

use crate::stage2::imports::{import_module, lower_hir_key};

pub fn lower_import_statement(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagPossible {
    if let ASTNodeKind::ImportStatement { path, kind } = node.kind.clone() {
        let path = lower_hir_key(path, file_ctx);

        let module = GLOBAL_STATE
            .with_borrow(|state| Ok(state.module_tree.traverse_to(path.clone(), &node)?.clone()))?;
        let module_inner;

        if let ModuleTreeEntry::Module(module) = module {
            module_inner = module.clone();
        } else {
            return Err(build_expected_entry_type(
                &"module".to_string(),
                &"???".to_string(),
                &node,
            )
            .into());
        }

        match kind {
            ImportKind::Whole => {
                import_module(module_inner, file_ctx.current_module.clone(), ctx, &node)?;
            }

            ImportKind::Module => {
                let mut path = file_ctx.current_module.clone();
                path.append_single_bit(path.last());

                import_module(module_inner, path, ctx, &node)?
            }

            _ => todo!(),
        };

        todo!()
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

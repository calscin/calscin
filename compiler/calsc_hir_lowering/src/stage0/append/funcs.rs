use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::file::HIRFileContext;
use calsc_modules::tree::{ModuleTree, entry::ModuleTreeEntry};

use crate::convert_visibility;

pub fn lower_stage_0_append_pass_function_declaratino(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments: _,
        return_type: _,
        body: _,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        if !visibility.can_be_imported() {
            return Ok(());
        }

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.append_single_bit(name);

        tree.traverse_to_append(path_to_append_to, ModuleTreeEntry::EmptyFunction, &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

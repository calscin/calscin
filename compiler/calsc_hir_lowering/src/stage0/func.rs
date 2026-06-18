use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::file::HIRFileContext;
use calsc_modules::tree::{ModuleTree, entry::ModuleTreeEntry};

use crate::{convert_visibility, stage0::types::lower_ast_type};

pub fn lower_ast_function_decl_stage_zero(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body: _,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        if !visibility.can_be_imported() {
            return Ok(());
        }

        let return_type = lower_ast_type(return_type, tree, file_ctx);
        let arguments: Vec<_> = arguments
            .iter()
            .map(|entry| {
                (
                    entry.1.clone(),
                    lower_ast_type(entry.0.clone(), tree, file_ctx),
                )
            })
            .collect();

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.path.push(name);

        let entry = ModuleTreeEntry::Function(return_type, arguments);

        tree.traverse_to_append(path_to_append_to, entry, &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_extern_func_decl_stage_zero(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    if let ASTNodeKind::ExternFunctionDeclaration {
        name,
        arguments,
        return_type,
        triple_dot_position: _,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        if !visibility.can_be_imported() {
            return Ok(());
        }

        let return_type = lower_ast_type(return_type, tree, file_ctx);
        let arguments: Vec<_> = arguments
            .iter()
            .map(|entry| {
                (
                    entry.1.clone(),
                    lower_ast_type(entry.0.clone(), tree, file_ctx),
                )
            })
            .collect();

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.path.push(name);

        let entry = ModuleTreeEntry::Function(return_type, arguments);

        tree.traverse_to_append(path_to_append_to, entry, &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

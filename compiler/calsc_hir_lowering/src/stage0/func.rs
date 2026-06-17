use calsc_ast::{
    nodes::{ASTNode, ASTNodeKind},
    path,
};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{build_expected_entry_type, build_internal_hir_node_leaked},
};
use calsc_hir::file::HIRFileContext;
use calsc_modules::tree::{ModuleTree, entry::ModuleTreeEntry};

use crate::{
    convert_visibility,
    stage0::types::{LazyLoadedTypeId, lower_ast_type},
};

pub fn lower_ast_function_decl_stage_zero(
    node: ASTNode,
    target: Option<LazyLoadedTypeId>,
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

        let return_type = lower_ast_type(return_type);
        let arguments: Vec<_> = arguments
            .iter()
            .map(|entry| (entry.1.clone(), lower_ast_type(entry.0.clone())))
            .collect();

        // Add the function to the type instead if there's a type linked to this function
        if target.is_some() {
            let target = target.unwrap();

            let mut path_to_mutate = target.0.clone();
            path_to_mutate.path.push(target.1);

            let entry = tree.traverse_mutably_to(path_to_mutate, &node)?;

            if let ModuleTreeEntry::Type(ty) = entry {
                ty.append_function(name, return_type, arguments, &node)?;
            } else {
                return Err(build_expected_entry_type(
                    &"type".to_string(),
                    &"??".to_string(),
                    &node,
                )
                .into());
            }

            return Ok(());
        }

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.path.push(name);

        let entry = ModuleTreeEntry::Function(return_type, arguments);

        tree.traverse_to_append(path_to_append_to, entry, &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

use std::collections::HashSet;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{
    DiagPossible,
    diags::errors::{build_already_in_scope, build_internal_hir_node_leaked},
};
use calsc_hir::file::HIRFileContext;
use calsc_modules::{
    lazy::func::LazyLoadedFunction,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};
use calsc_typing::ctx::TypeCtx;

use crate::{convert_visibility, stage0::fill::types::lower_ast_type};

pub fn lower_ast_function_decl_stage_zero(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
    type_ctx: &mut TypeCtx,
) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body: _,
        visibility,
        type_parameters,
    } = node.kind.clone()
    {
        let group = type_ctx.type_params.start_param_group();
        let mut type_params = HashSet::new();

        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        if !visibility.can_be_imported() {
            return Ok(());
        }

        // Import type parameters
        for param in type_parameters {
            if type_params.contains(&param) {
                return Err(build_already_in_scope(&param, &node).into());
            }

            type_ctx
                .type_params
                .append_type_param(param.clone(), &node)?;

            type_params.insert(param);
        }

        let return_type = lower_ast_type(return_type, tree, file_ctx, type_ctx);
        let arguments: Vec<_> = arguments
            .iter()
            .map(|entry| {
                (
                    entry.1.clone(),
                    lower_ast_type(entry.0.clone(), tree, file_ctx, type_ctx),
                )
            })
            .collect();

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.path.push(name);

        let mut func = LazyLoadedFunction::new(return_type, arguments);
        func.type_paramers = type_params;

        let entry = ModuleTreeEntry::FilledFunction(func);

        type_ctx.type_params.end_group(group);

        tree.traverse_to_append(path_to_append_to, entry, &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_extern_func_decl_stage_zero(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
    type_ctx: &TypeCtx,
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

        let return_type = lower_ast_type(return_type, tree, file_ctx, type_ctx);
        let arguments: Vec<_> = arguments
            .iter()
            .map(|entry| {
                (
                    entry.1.clone(),
                    lower_ast_type(entry.0.clone(), tree, file_ctx, type_ctx),
                )
            })
            .collect();

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.path.push(name);

        let entry =
            ModuleTreeEntry::FilledFunction(LazyLoadedFunction::new(return_type, arguments));

        tree.traverse_to_append(path_to_append_to, entry, &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

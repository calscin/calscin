//! The file walking / processing stage.
//! Basically processes the AST context in order to build the module tree

use calsc_ast::{ASTContext, nodes::ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_modules::treev2::entry::TreeEntryKind;
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{ctx::TreeBuildingCtx, walk::modules::walk_through_module};

pub mod modules;

pub fn walk_in_file(ast: &ASTContext, ctx: &mut TreeBuildingCtx) -> DiagPossible {
    for node in &ast.tree {
        walk_through_node(node, ast, ctx)?;
    }

    Ok(())
}

pub fn walk_through_node(
    node: &ArenaHandle,
    ast: &ASTContext,
    ctx: &mut TreeBuildingCtx,
) -> DiagPossible {
    let node_ref = ast.nodes.get(node);

    match &node_ref.kind {
        ASTNodeKind::Module { .. } => walk_through_module(node, ast, ctx),

        ASTNodeKind::StructDeclaration {
            name,
            fields: _,
            visibility: _,
            type_parameters: _,
        } => {
            let mut path_to_append_to = ctx.current_path.clone();
            path_to_append_to.append_single_bit(name.clone());

            ctx.tree.append_entry(
                &path_to_append_to,
                TreeEntryKind::Type,
                &mut ctx.arena,
                node_ref,
            )?;

            ctx.append_related_node(
                path_to_append_to,
                ctx.current_file.clone(),
                node_ref.clone(),
            );
            Ok(())
        }

        ASTNodeKind::ExternFunctionDeclaration {
            name,
            arguments: _,
            return_type: _,
            triple_dot_position: _,
            visibility: _,
        } => {
            let mut path_to_append_to = ctx.current_path.clone();
            path_to_append_to.append_single_bit(name.clone());

            ctx.tree.append_entry(
                &path_to_append_to,
                TreeEntryKind::Function,
                &mut ctx.arena,
                node_ref,
            )?;

            ctx.append_related_node(
                path_to_append_to,
                ctx.current_file.clone(),
                node_ref.clone(),
            );
            Ok(())
        }

        ASTNodeKind::FunctionDeclaration {
            name,
            arguments: _,
            return_type: _,
            body: _,
            visibility: _,
            type_parameters: _,
        } => {
            let mut path_to_append_to = ctx.current_path.clone();
            path_to_append_to.append_single_bit(name.clone());

            ctx.tree.append_entry(
                &path_to_append_to,
                TreeEntryKind::Function,
                &mut ctx.arena,
                node_ref,
            )?;

            ctx.append_related_node(
                path_to_append_to,
                ctx.current_file.clone(),
                node_ref.clone(),
            );
            Ok(())
        }

        ASTNodeKind::ImportStatement { .. } => {
            println!("Adding import node to {}", ctx.current_path);
            ctx.append_import_node(node_ref.clone());

            Ok(())
        } // Handle this in second pass

        _ => return Err(build_internal_hir_node_leaked(node_ref, node_ref).into()),
    }
}

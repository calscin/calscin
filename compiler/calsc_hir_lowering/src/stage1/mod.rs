//! Declarations for the first stage of the HIR lowering. The stage 1 has a couple of responsibilities:
//! - Add types to the global scope
//! - Add stage 1 functions to the global scope
//! - Add extern function to the global scope
//!
//! The stage 1 should only create the local context and append the arguments inside

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_modules::visibility::Visibility;
use calsc_typing_v2::prelude::apply_prelude;

use crate::stage1::{
    funcs::{lower_ast_extern_function, lower_ast_function_decl_first_stage},
    types::lower_ast_struct_declaration,
};

pub mod funcs;
pub mod types;

pub fn lower_hir_stage_1_node(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagPossible {
    match node.kind {
        ASTNodeKind::FunctionDeclaration { .. } => {
            lower_ast_function_decl_first_stage(ASTNode::clone(&node), file_ctx, ctx)?;
        }

        ASTNodeKind::ExternFunctionDeclaration { .. } => {
            lower_ast_extern_function(ASTNode::clone(&node), file_ctx, ctx)?
        }
        ASTNodeKind::StructDeclaration { .. } => {
            lower_ast_struct_declaration(ASTNode::clone(&node), file_ctx, ctx)?
        }

        ASTNodeKind::Module { .. } => lower_hir_stage_1_module(node, file_ctx, ctx, ast_ctx)?,

        ASTNodeKind::ImportStatement { .. } => {}

        _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
    };

    Ok(())
}

pub fn lower_hir_stage_1(
    ast_context: ASTContext,
    ctx: &mut HIRContext,
    file_ctx: &mut HIRFileContext,
) -> DiagPossible {
    let mut first = false;

    for node in &ast_context.tree {
        if !first {
            first = true;

            apply_prelude(&mut ctx.scope, ast_context.nodes.get(node))?;
        }

        lower_hir_stage_1_node(
            ASTNode::clone(&ast_context.nodes.get(node)),
            file_ctx,
            ctx,
            &ast_context,
        )?;
    }

    Ok(())
}

pub fn lower_hir_stage_1_module(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagPossible {
    if let ASTNodeKind::Module {
        name,
        body,
        is_bodied,
    } = node.kind.clone()
    {
        if !is_bodied {
            let key =
                GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

            let mut path = file_ctx.current_module.clone();
            path.path.push(name);

            ctx.scope.append(
                key,
                GlobalContextValue::Module(path),
                Visibility::Public,
                &node,
            )?;

            return Ok(());
        }

        file_ctx.advance_module(name);

        for element in body {
            lower_hir_stage_1_node(
                ASTNode::clone(&ast_ctx.nodes.get(&element)),
                file_ctx,
                ctx,
                ast_ctx,
            )?;
        }

        file_ctx.deadvance_module();

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

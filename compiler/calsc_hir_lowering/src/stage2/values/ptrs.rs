use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible, DiagResult,
    diags::errors::{build_expected_referencable, build_internal_hir_node_leaked},
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_typing_v2::types::MutationState;
use calsc_utils::alloc::arena::ArenaHandle;

use crate::stage2::values::lower_ast_value;

pub fn introduce_reference_ast(
    node: ArenaHandle,
    local_ctx: Option<GlobalContextKey>,
    ctx: &mut HIRContext,
) -> DiagPossible {
    let ind = ctx.nodes.get(&node).get_root_variable_reference_index(ctx);

    let node = ctx.nodes.get(&node).clone();

    ctx.scope.mutate_entry(
        local_ctx.unwrap(),
        |entry| {
            entry.mutate_function(
                |ff| {
                    ff.local_context.as_mut().unwrap().variables[ind].introduce_reference();
                },
                &node,
            )
        },
        &node,
    )??;

    Ok(())
}

pub fn lower_ast_pointer_reference(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::PointerReference(val, mutable) = node.kind.clone() {
        let val_ref = ast_ctx.nodes.get(&val);

        let val = lower_ast_value(val_ref.clone(), local_ctx.clone(), file_ctx, ctx, ast_ctx)?;

        if !ctx.nodes.get(&val).represents_pointer_referencable(ctx) {
            return Err(build_expected_referencable(&node).into());
        }

        introduce_reference_ast(val.clone(), local_ctx.clone(), ctx)?;

        let node = HIRNode::new(
            HIRNodeKind::PointerReference(val, MutationState(mutable)),
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_pointer_dereference(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::PointerDereference(val) = node.kind.clone() {
        let val = lower_ast_value(
            ast_ctx.nodes.get(&val).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let val_ref = ctx.nodes.get(&val);

        if !val_ref.represents_pointer_referencable(ctx) {
            return Err(build_expected_referencable(&node).into());
        }

        let node = HIRNode::new(
            HIRNodeKind::PointerDereference(val),
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_cannot_find_element_no_closest, build_internal_hir_node_leaked},
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};

use calsc_typing_v2::traits::FieldedType;
use calsc_utils::alloc::arena::ArenaHandle;

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_lru(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::StructLRUsage {
        left_expr,
        right_expr,
    } = node.kind.clone()
    {
        let left_expr = lower_ast_value(
            ASTNode::clone(&ast_ctx.nodes.get(&left_expr)),
            curr_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let left_expr_ref = ctx.nodes.get(&left_expr).clone();

        let left_ty = left_expr_ref.get_type(curr_ctx.clone(), ctx, Some(file_ctx))?;

        let right_expr_ref = ast_ctx.nodes.get(&right_expr);

        match &right_expr_ref.kind {
            ASTNodeKind::ElementReference(name) => {
                if !left_ty.has_field(&name, &ctx.type_ctx) {
                    return Err(build_cannot_find_element_no_closest(&name, &node).into());
                }

                let field_ind = left_ty.get_field_index(&name, &ctx.type_ctx);

                let node = HIRNode::new(
                    HIRNodeKind::FieldReference {
                        val: left_expr,
                        field_ind,
                        name: name.clone(),
                    },
                    node.start.clone(),
                    node.end.clone(),
                );

                return Ok(node.push(ctx));
            }

            _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
        }
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

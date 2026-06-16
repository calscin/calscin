use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{
        build_cannot_find_element_no_closest, build_cannot_parse_error,
        build_internal_hir_node_leaked,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_typing::{FieldHavingType, func::DeclBlockAffectedType};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::stage2::{funcs::lower_ast_function_call, values::lower_ast_value};

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

        let left_ty = ctx
            .nodes
            .get(&left_expr)
            .get_type(curr_ctx.clone(), ctx, file_ctx)?;

        let right_expr_ref = ast_ctx.nodes.get(&right_expr);

        match &right_expr_ref.kind {
            ASTNodeKind::FunctionCall { name, arguments: _ } => {
                if name.members.len() != 1 {
                    return Err(build_cannot_parse_error(
                        &"LRU function call".to_string(),
                        right_expr_ref,
                    )
                    .into());
                }

                if !left_ty.clone().has_function(name.last().clone()) {
                    return Err(build_cannot_find_element_no_closest(&name, &node).into());
                }

                if !left_ty.is_transparent_real() {
                    return Err(build_cannot_find_element_no_closest(&name, &node).into());
                }

                let ret = lower_ast_function_call(
                    node,
                    Some(left_ty.get_transparent_real().ty),
                    curr_ctx.clone(),
                    file_ctx,
                    ctx,
                    ast_ctx,
                )?;

                let ret = ctx.nodes.get(&ret).clone();

                if let HIRNodeKind::FunctionCall { func, arguments } = ret.kind.clone() {
                    let mut arguments = arguments;

                    arguments.insert(0, left_expr);

                    let ret = HIRNode::new(
                        HIRNodeKind::FunctionCall { func, arguments },
                        ret.start.clone(),
                        ret.end.clone(),
                    );

                    return Ok(ret.push(ctx));
                } else {
                    return Err(build_internal_hir_node_leaked(&ret, &ret).into());
                }
            }

            ASTNodeKind::ElementReference(name) => {
                if !left_ty.has_field(name.clone()) {
                    return Err(build_cannot_find_element_no_closest(&name, &node).into());
                }

                let field_ind = left_ty.get_field_index(name.clone());

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

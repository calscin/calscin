use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_expected_type_error, build_internal_hir_node_leaked},
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_typing::types::primitive::PrimitiveType;
use calsc_utils::{alloc::arena::ArenaHandle, display_with_to_string};

use crate::{
    stage1::types::lower_ast_type,
    stage2::{funcs::lower_ast_body, values::lower_ast_value},
};

pub fn lower_ast_matches(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::MatchBlock {
        val,
        matches,
        default_match,
    } = node.kind.clone()
    {
        let val = lower_ast_value(
            ast_ctx.nodes.get(&val).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;
        let val_type =
            ctx.nodes
                .get(&val)
                .clone()
                .get_type(local_ctx.clone(), ctx, Some(file_ctx))?;

        if !val_type.has_direct_primitive(&ctx.type_ctx) {
            let ty = val_type.get_primitive(&ctx.type_ctx);

            if !ty.ty.is_enum() {
                return Err(build_expected_type_error(
                    &"enum type".to_string(),
                    &display_with_to_string(&val_type, &ctx.type_ctx),
                    &node,
                )
                .into());
            }
        }

        let primitive_container = match val_type.get_primitive(&ctx.type_ctx).ty {
            PrimitiveType::Enum(container) => container.clone(),
            _ => unreachable!(),
        };

        // We first handle the branches
        let mut branches = vec![];

        for (branch_ty, unwrap) in matches {
            let branch_ty = lower_ast_type(branch_ty, &node, file_ctx, ctx)?;

            if !branch_ty.is_directly_primitive() || !branch_ty.as_primitive().ty.is_enum_entry() {
                return Err(build_expected_type_error(
                    &"enum entry".to_string(),
                    &display_with_to_string(&branch_ty, &ctx.type_ctx),
                    &node,
                )
                .into());
            }

            if let PrimitiveType::EnumEntry(container, _) = branch_ty.as_primitive().ty {
                if container != primitive_container {
                    return Err(build_expected_type_error(
                        &"enum entry of the type".to_string(),
                        &display_with_to_string(&branch_ty, &ctx.type_ctx),
                        &node,
                    )
                    .into());
                }
            };

            let variable_index = ctx.scope.mutate_entry(
                local_ctx.clone().unwrap(),
                |entry| {
                    entry.mutate_function(
                        |ff| {
                            ff.local_context
                                .as_mut()
                                .unwrap()
                                .introduce_variable_next_branch(
                                    unwrap.0.clone(),
                                    branch_ty.clone(),
                                    true,
                                    true,
                                    &node,
                                )
                        },
                        &node,
                    )
                },
                &node,
            )???;

            let body = lower_ast_body(
                unwrap
                    .1
                    .iter()
                    .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                    .collect(),
                local_ctx.clone(),
                true,
                &node,
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            branches.push((branch_ty.as_primitive().ty, unwrap.0, variable_index, body))
        }

        // Handle default branch
        let mut default_branch = None;

        if default_match.is_some() {
            default_branch = Some(lower_ast_body(
                default_match
                    .unwrap()
                    .iter()
                    .map(|f| ASTNode::clone(&ast_ctx.nodes.get(f)))
                    .collect(),
                local_ctx,
                true,
                &node,
                file_ctx,
                ctx,
                ast_ctx,
            )?);
        }

        let start = node.start.clone();
        let end = node.end.clone();

        let node = HIRNode::new(
            HIRNodeKind::MatchBlock {
                val,
                matches: branches,
                default_match: default_branch,
            },
            start,
            end,
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

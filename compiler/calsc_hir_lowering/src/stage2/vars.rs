//! Variable lowering

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagPossible, DiagResult,
    diags::errors::{build_expected_mutable, build_internal_hir_node_leaked},
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    types::validate_type_for_storage,
};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{stage1::types::lower_ast_type, stage2::values::lower_ast_value};

pub fn lower_ast_variable_reference(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
    ctx: &mut HIRContext,
    file_ctx: &HIRFileContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::ElementReference(val) = &node.kind {
        let ind = ctx
            .scope
            .get_entry(curr_ctx.unwrap(), &file_ctx.current_module, &node)
            .unwrap()
            .as_function(&node)
            .unwrap()
            .local_context
            .as_ref()
            .unwrap()
            .obtain(val.clone(), &node)?;

        let node = HIRNode::new(
            HIRNodeKind::VariableReference {
                name: val.clone(),
                variable_index: ind,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_variable_declaration(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::VariableDeclaration {
        mutable,
        var_type,
        name,
        value,
    } = node.kind.clone()
    {
        let var_type = lower_ast_type(var_type, &node, None, file_ctx, ctx)?;

        validate_type_for_storage(&var_type, &node)?;

        let id = ctx.scope.mutate_entry(
            curr_ctx.clone().unwrap(),
            |entry| {
                entry.mutate_function(
                    |ff| {
                        let local_ctx = ff.local_context.as_mut().unwrap();
                        local_ctx.introduce_variable(
                            name.clone(),
                            var_type.clone(),
                            value.is_some(),
                            &node,
                        )
                    },
                    &node,
                )?
            },
            &node,
        )??;

        let mut v = None;

        if value.is_some() {
            let value = lower_ast_value(
                ASTNode::clone(ast_ctx.nodes.get(value.as_ref().unwrap())),
                curr_ctx.clone(),
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            let value_ref = ctx.nodes.get(&value).clone();

            v = Some(
                value_ref
                    .use_as(
                        var_type.clone(),
                        value.clone(),
                        None,
                        curr_ctx.clone(),
                        ctx,
                        file_ctx,
                    )?
                    .push(ctx),
            );
        }

        let node = HIRNode::new(
            HIRNodeKind::VariableDeclaration {
                mutable,
                var_type,
                value: v,
                name,
                variable_index: id,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn introduce_variable_mutation(
    node: ArenaHandle,
    curr_ctx: Option<GlobalContextKey>,
    ctx: &mut HIRContext,
) -> DiagPossible {
    let ind = ctx.nodes.get(&node).get_root_variable_reference_index(ctx);

    let node = HIRNode::clone(ctx.nodes.get(&node));

    ctx.scope.mutate_entry(
        curr_ctx.unwrap(),
        |entry| {
            entry.mutate_function(
                |ff| {
                    ff.local_context.as_mut().unwrap().variables[ind].introduce_mutation();
                    let current_branch = ff.local_context.as_ref().unwrap().current_branch;

                    ff.local_context.as_mut().unwrap().variables[ind]
                        .introduced_values
                        .insert(current_branch);
                },
                &node,
            )
        },
        &node,
    )??;

    Ok(())
}

pub fn lower_ast_variable_assign(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::Assignment { variable, value } = node.kind.clone() {
        let variable = lower_ast_value(
            ASTNode::clone(ast_ctx.nodes.get(&variable)),
            curr_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let variable_ref = ctx.nodes.get(&variable).clone();
        let variable_type = variable_ref.get_type(curr_ctx.clone(), ctx, Some(file_ctx))?;

        let value = lower_ast_value(
            ast_ctx.nodes.get(&value).clone(),
            curr_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let value_ref = ctx.nodes.get(&value).clone();

        let value = value_ref
            .use_as(
                variable_type,
                value.clone(),
                None,
                curr_ctx.clone(),
                ctx,
                file_ctx,
            )?
            .push(ctx);

        if !ctx.nodes.get(&variable).represents_mutable_variable(ctx) {
            return Err(build_expected_mutable(&variable_ref).into());
        }

        introduce_variable_mutation(variable.clone(), curr_ctx.clone(), ctx)?;

        let n = HIRNodeKind::Assignment {
            variable: variable.clone(),
            value: value.clone(),
        };

        let node = HIRNode::new(n, node.start.clone(), node.end.clone());

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

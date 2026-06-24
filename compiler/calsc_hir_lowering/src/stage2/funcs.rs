use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{
        build_expected_entry_type, build_expected_return_error, build_internal_hir_node_leaked,
        build_restricted_return_type,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_typing::types::TypeKind;
use calsc_utils::{alloc::arena::ArenaHandle, display_with_to_string};

use crate::{
    stage1::types::lower_ast_type,
    stage2::{
        control::{
            lower_ast_for_loop, lower_ast_if_statement, lower_ast_loop, lower_ast_while_loop,
        },
        key::lower_ast_key,
        values::{lower_ast_value, lru::lower_ast_lru},
        vars::{lower_ast_variable_assign, lower_ast_variable_declaration},
    },
};

pub fn lower_ast_body_node(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    match &node.kind {
        ASTNodeKind::FunctionCall { .. } => {
            lower_ast_function_call(node, local_ctx, file_ctx, ctx, ast_ctx)
        }
        ASTNodeKind::StructLRUsage { .. } => lower_ast_lru(node, local_ctx, file_ctx, ctx, ast_ctx),

        ASTNodeKind::VariableDeclaration { .. } => {
            lower_ast_variable_declaration(node, local_ctx, file_ctx, ctx, ast_ctx)
        }
        ASTNodeKind::Assignment { .. } => {
            lower_ast_variable_assign(node, local_ctx, file_ctx, ctx, ast_ctx)
        }

        ASTNodeKind::IfStatement { .. } => {
            lower_ast_if_statement(node, local_ctx, file_ctx, ctx, ast_ctx)
        }
        ASTNodeKind::ForLoop { .. } => lower_ast_for_loop(node, local_ctx, file_ctx, ctx, ast_ctx),
        ASTNodeKind::WhileLoop { .. } => {
            lower_ast_while_loop(node, local_ctx, file_ctx, ctx, ast_ctx)
        }
        ASTNodeKind::Loop { .. } => lower_ast_loop(node, local_ctx, file_ctx, ctx, ast_ctx),

        ASTNodeKind::ReturnStatement { .. } => {
            lower_ast_return_statement(node, local_ctx, file_ctx, ctx, ast_ctx)
        }

        _ => lower_ast_value(node, local_ctx, file_ctx, ctx, ast_ctx),
    }
}

pub fn lower_ast_body<K: DiagnosticSource>(
    nodes: Vec<ASTNode>,
    local_ctx: Option<GlobalContextKey>,
    introduce_branch: bool,
    origin: &K,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<Vec<ArenaHandle>> {
    let mut hir_nodes = vec![];

    let previous_branch = ctx
        .scope
        .get_entry(local_ctx.clone().unwrap(), &file_ctx.current_module, origin)?
        .as_function(origin)?
        .local_context
        .as_ref()
        .unwrap()
        .current_branch;

    let mut branch = 0;

    if introduce_branch {
        branch = ctx.scope.mutate_entry(
            local_ctx.clone().unwrap(),
            |entry| {
                entry.mutate_function(
                    |ff| ff.local_context.as_mut().unwrap().start_branch(),
                    origin,
                )
            },
            origin,
        )??;
    }

    for node in &nodes {
        hir_nodes.push(lower_ast_body_node(
            node.clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?);
    }

    if introduce_branch {
        ctx.scope.mutate_entry(
            local_ctx.unwrap(),
            |entry| {
                entry.mutate_function(
                    |ff| {
                        ff.local_context
                            .as_mut()
                            .unwrap()
                            .end_branch(branch, origin);

                        ff.local_context.as_mut().unwrap().current_branch = previous_branch;
                    },
                    origin,
                )
            },
            origin,
        )??;
    }

    Ok(hir_nodes)
}

pub fn lower_ast_function_call(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::FunctionCall { name, arguments } = node.kind.clone() {
        let key = lower_ast_key(name, &node, true, file_ctx, ctx)?;

        let mut hir_arguments = vec![];

        for argument in arguments {
            hir_arguments.push(lower_ast_value(
                ASTNode::clone(ast_ctx.nodes.get(&argument)),
                local_ctx.clone(),
                file_ctx,
                ctx,
                ast_ctx,
            )?);
        }

        let is_function = ctx
            .scope
            .get_entry(key.clone(), &file_ctx.current_module, &node)?
            .is_function();

        if !is_function {
            return Err(build_expected_entry_type(&"function", &"?? TODO", &node).into());
        }

        let node = HIRNode::new(
            HIRNodeKind::FunctionCall {
                func: key,
                arguments: hir_arguments,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_return_statement(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::ReturnStatement { val } = node.kind.clone() {
        let v;

        let expected_return_type = ctx
            .scope
            .get_entry(local_ctx.clone().unwrap(), &file_ctx.current_module, &node)?
            .as_function(&node)?
            .local_context
            .as_ref()
            .unwrap()
            .return_type
            .clone();

        if val.is_some() && expected_return_type == TypeKind::Void {
            return Err(build_restricted_return_type(&"void", &node).into());
        }

        if val.is_some() {
            let val = lower_ast_value(
                ASTNode::clone(ast_ctx.nodes.get(val.as_ref().unwrap())),
                local_ctx.clone(),
                file_ctx,
                ctx,
                ast_ctx,
            )?;

            let val_ref = ctx.nodes.get(&val).clone();

            let val = val_ref
                .use_as(
                    &expected_return_type,
                    val.clone(),
                    None,
                    local_ctx.clone(),
                    ctx,
                    file_ctx,
                )?
                .push(ctx);

            v = Some(val);
        } else {
            v = None;
        }

        ctx.scope.mutate_entry(
            local_ctx.clone().unwrap(),
            |entry| {
                entry.mutate_function(
                    |ff| ff.local_context.as_mut().unwrap().introduce_ending_point(),
                    &node,
                )
            },
            &node,
        )??;

        let node = HIRNode::new(
            HIRNodeKind::ReturnStatement { val: v },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_function_decl(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body,
        visibility: _,
    } = node.kind.clone()
    {
        let mut key =
            GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

        let is_main_function = key.name == "main".into() && key.module_path.path.len() == 0;

        if is_main_function {
            key = GlobalContextKey::new("main".into());
        }

        let mut hir_arguments = vec![];
        let ret_type = lower_ast_type(return_type, &node, file_ctx, ctx)?;

        for argument in arguments {
            hir_arguments.push((
                lower_ast_type(argument.0.clone(), &node, file_ctx, ctx)?,
                argument.1,
            ));
        }

        let body = lower_ast_body(
            body.iter()
                .map(|f| ASTNode::clone(ast_ctx.nodes.get(f)))
                .collect(),
            Some(key.clone()),
            false,
            &node,
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let meets_ending_point = ctx
            .scope
            .get_entry(key.clone(), &file_ctx.current_module, &node)?
            .as_function(&node)?
            .local_context
            .as_ref()
            .unwrap()
            .meets_ending_point_requirement();

        if !meets_ending_point {
            return Err(build_expected_return_error(
                &display_with_to_string(&ret_type, &ctx.type_ctx),
                &"void".to_string(),
                &node,
            )
            .into());
        }

        let is_void = ret_type == TypeKind::Void;

        let n = HIRNode::new(
            HIRNodeKind::FunctionDeclaration {
                key: key.clone(),
                arguments: hir_arguments,
                body,
                return_type: ret_type,
                append_terminator: key == GlobalContextKey::new("main".into()) && is_void,
            },
            node.start.clone(),
            node.end.clone(),
        );

        let r = n.push(ctx);

        ctx.scope.mutate_entry(
            key.clone(),
            |entry| entry.mutate_function(|ff| ff.impl_node = Some(r.clone()), &node),
            &node,
        )??;

        Ok(r)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

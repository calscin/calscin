use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagResult, DiagnosticSource, diags::errors::build_expected_error};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use calsc_typing::base::BaseType;

use crate::{
    stage1::types::lower_ast_type,
    stage2::{
        control::{
            lower_ast_for_loop, lower_ast_if_statement, lower_ast_loop, lower_ast_while_loop,
        },
        values::{lower_ast_value, lru::lower_ast_lru},
        vars::{lower_ast_variable_assign, lower_ast_variable_declaration},
    },
};

pub fn lower_ast_body_node(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    match &node.kind {
        ASTNodeKind::FunctionCall { .. } => lower_ast_function_call(node, None, local_ctx),
        ASTNodeKind::StructLRUsage { .. } => lower_ast_lru(node, local_ctx),

        ASTNodeKind::VariableDeclaration { .. } => lower_ast_variable_declaration(node, local_ctx),
        ASTNodeKind::Assignment { .. } => lower_ast_variable_assign(node, local_ctx),

        ASTNodeKind::IfStatement { .. } => lower_ast_if_statement(node, local_ctx),
        ASTNodeKind::ForLoop { .. } => lower_ast_for_loop(node, local_ctx),
        ASTNodeKind::WhileLoop { .. } => lower_ast_while_loop(node, local_ctx),
        ASTNodeKind::Loop { .. } => lower_ast_loop(node, local_ctx),

        ASTNodeKind::ReturnStatement { .. } => lower_ast_return_statement(node, local_ctx),

        _ => lower_ast_value(node, local_ctx),
    }
}

pub fn lower_ast_body<K: DiagnosticSource>(
    nodes: Vec<ASTNode>,
    local_ctx: Option<GlobalContextKey>,
    introduce_branch: bool,
    origin: &K,
) -> DiagResult<Vec<HIRArenaReference>> {
    let mut hir_nodes = vec![];

    let previous_branch = HIR_CONTEXT.with(|f| {
        Ok(f.borrow()
            .scope
            .get_entry(local_ctx.clone().unwrap(), origin)?
            .as_function(origin)?
            .local_context
            .as_ref()
            .unwrap()
            .current_branch)
    })?;

    let mut branch = 0;
    let mut last = None;

    if introduce_branch {
        branch = HIR_CONTEXT.with(|f| {
            Ok(f.borrow_mut().scope.mutate_entry(
                local_ctx.clone().unwrap(),
                |entry| {
                    entry.mutate_function(
                        |ff| ff.local_context.as_mut().unwrap().start_branch(),
                        origin,
                    )
                },
                origin,
            )?)
        })??;
    }

    for node in &nodes {
        last = Some(node.clone());
        hir_nodes.push(lower_ast_body_node(node.clone(), local_ctx.clone())?);
    }

    if introduce_branch {
        HIR_CONTEXT.with(|f| {
            f.borrow_mut().scope.mutate_entry(
                local_ctx.unwrap(),
                |entry| {
                    entry.mutate_function(
                        |ff| {
                            ff.local_context
                                .as_mut()
                                .unwrap()
                                .end_branch(branch, &last.unwrap());
                            ff.local_context.as_mut().unwrap().current_branch = previous_branch;
                        },
                        origin,
                    )
                },
                origin,
            )
        })??;
    }

    Ok(hir_nodes)
}

pub fn lower_ast_function_call(
    node: ASTNode,
    ty: Option<BaseType>,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::FunctionCall { name, arguments } = node.kind.clone() {
        let key;
        let mut hir_arguments = vec![];

        if ty.is_some() {
            key = GlobalContextKey::new_typed(name, ty.unwrap());
        } else {
            key = GlobalContextKey::new(name);
        }

        for argument in arguments {
            hir_arguments.push(lower_ast_value(
                ASTNode::clone(&argument),
                local_ctx.clone(),
            )?);
        }

        let is_function = HIR_CONTEXT.with(|f| {
            Ok(f.borrow()
                .scope
                .get_entry(key.clone(), &node)?
                .is_function())
        });

        if !is_function? {
            return Err(build_expected_error(&"function", &"?? TODO", &node).into());
        }

        let node = HIRNode::new(
            HIRNodeKind::FunctionCall {
                func: key,
                arguments: hir_arguments,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_return_statement(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::ReturnStatement { val } = node.kind.clone() {
        let v;

        if val.is_some() {
            v = Some(lower_ast_value(
                ASTNode::clone(&val.unwrap()),
                local_ctx.clone(),
            )?);
        } else {
            v = None;
        }

        let node = HIRNode::new(
            HIRNodeKind::ReturnStatement { val: v },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_function_decl(
    node: ASTNode,
    ty: Option<BaseType>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::FunctionDeclaration {
        name,
        arguments,
        return_type,
        body,
    } = node.kind.clone()
    {
        let key;

        if ty.is_some() {
            key = GlobalContextKey::new_typed(name.clone(), ty.clone().unwrap());
        } else {
            key = GlobalContextKey::new(name.clone());
        }

        println!("Key: {}", key);

        let mut hir_arguments = vec![];
        let ret_type;

        for argument in arguments {
            hir_arguments.push((
                lower_ast_type(argument.0.clone(), &node, ty.clone())?,
                argument.1,
            ));
        }

        if return_type.is_some() {
            ret_type = Some(lower_ast_type(return_type.unwrap(), &node, ty.clone())?);
        } else {
            ret_type = None;
        }

        let body = lower_ast_body(
            body.iter().map(|f| ASTNode::clone(f)).collect(),
            Some(key.clone()),
            false,
            &node,
        )?;

        let n = HIRNode::new(
            HIRNodeKind::FunctionDeclaration {
                key: key.clone(),
                arguments: hir_arguments,
                body,
                return_type: ret_type,
            },
            node.start.clone(),
            node.end.clone(),
        );

        let r = n.push();

        HIR_CONTEXT.with(|f| {
            f.borrow_mut().scope.mutate_entry(
                key.clone(),
                |entry| entry.mutate_function(|ff| ff.impl_node = Some(r.clone()), &node),
                &node,
            )
        })??;

        Ok(r)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

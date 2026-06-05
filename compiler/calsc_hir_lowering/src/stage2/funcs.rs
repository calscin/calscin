use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagResult, diags::errors::build_expected_error};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use calsc_typing::base::BaseType;

use crate::{
    stage1::types::lower_ast_type,
    stage2::values::{lower_ast_value, lru::lower_ast_lru},
};

pub fn lower_ast_body_node(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    match &node.kind {
        ASTNodeKind::FunctionCall { .. } => lower_ast_function_call(node, None, local_ctx),
        ASTNodeKind::StructLRUsage { .. } => lower_ast_lru(node, local_ctx),

        _ => panic!(),
    }
}

pub fn lower_ast_body(
    nodes: Vec<ASTNode>,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<Vec<HIRArenaReference>> {
    let mut hir_nodes = vec![];

    for node in nodes {
        hir_nodes.push(lower_ast_body_node(node, local_ctx.clone())?);
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

        let is_function =
            HIR_CONTEXT.with_borrow(|f| Ok(f.scope.get_entry(key.clone(), &node)?.is_function()));

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
        let key = GlobalContextKey::new(name.clone());
        let mut hir_arguments = vec![];
        let ret_type;

        for argument in arguments {
            hir_arguments.push((
                lower_ast_type(argument.0.clone(), &node, ty.clone())?,
                argument.1,
            ));
        }

        if return_type.is_some() {
            ret_type = Some(lower_ast_type(return_type.unwrap(), &node, None)?);
        } else {
            ret_type = None;
        }

        let body = lower_ast_body(
            body.iter().map(|f| ASTNode::clone(f)).collect(),
            Some(key.clone()),
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

        HIR_CONTEXT.with_borrow_mut(|f| {
            f.scope.mutate_entry(
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

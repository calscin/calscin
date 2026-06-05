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

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_body_node(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    match &node.kind {
        ASTNodeKind::FunctionCall { .. } => lower_ast_function_call(node, None, local_ctx),

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

        let is_function = HIR_CONTEXT
            .with_borrow(|f| f.scope.get_entry(key.clone(), &node).unwrap().is_function());

        if !is_function {
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

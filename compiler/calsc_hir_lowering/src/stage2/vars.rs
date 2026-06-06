//! Variable lowering

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagResult, diags::errors::build_expected_error};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

use crate::{stage1::types::lower_ast_type, stage2::values::lower_ast_value};

#[deprecated = "will be changed to lower_ast_variable_reference"]
pub fn lower_hir_variable_reference(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::ElementReference(val) = &node.kind {
        let res = HIR_CONTEXT.with_borrow(|f| {
            f.scope
                .get_entry(curr_ctx.unwrap(), &node)
                .unwrap()
                .as_function(&node)
                .unwrap()
                .local_context
                .as_ref()
                .unwrap()
                .obtain(val.clone(), &node)
        });

        let ind = res?;

        let node = HIRNode::new(
            HIRNodeKind::VariableReference {
                name: val.clone(),
                variable_index: ind,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_variable_declaration(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::VariableDeclaration {
        mutable,
        var_type,
        name,
        value,
    } = node.kind.clone()
    {
        let var_type = lower_ast_type(var_type, &node, None)?;

        let id = HIR_CONTEXT.with_borrow_mut(|f| {
            f.scope.mutate_entry(
                curr_ctx.unwrap(),
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
                    )
                },
                &node,
            )
        })???;

        let node = HIRNode::new(
            HIRNodeKind::VariableDeclaration {
                mutable,
                var_type,
                name,
                variable_index: id,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_variable_assign(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::Assignment { variable, value } = node.kind.clone() {
        let variable = lower_ast_value(ASTNode::clone(&variable), curr_ctx.clone())?;
        let value = lower_ast_value(ASTNode::clone(&value), curr_ctx.clone())?
            .use_as(
                variable.get_type(curr_ctx.clone())?.unwrap(),
                curr_ctx.clone(),
            )?
            .push();

        if !variable.represents_mutable_variable() {
            return Err(build_expected_error(
                &"mutable variable-like".to_string(),
                &"".to_string(),
                &*variable,
            )
            .into());
        }

        let n;

        if let HIRNodeKind::PointerDereference(inner) = variable.kind.clone() {
            n = HIRNodeKind::PointerDerefAssign {
                pointer: inner,
                value,
            }
        } else {
            n = HIRNodeKind::Assignment { variable, value }
        }

        let node = HIRNode::new(n, node.start.clone(), node.end.clone());

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

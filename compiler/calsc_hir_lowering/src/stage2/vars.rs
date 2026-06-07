//! Variable lowering

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, DiagResult, diags::errors::build_expected_error};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

use crate::{stage1::types::lower_ast_type, stage2::values::lower_ast_value};

pub fn lower_ast_variable_reference(
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
                    )
                },
                &node,
            )
        })???;

        let mut v = None;

        if value.is_some() {
            let value = lower_ast_value(ASTNode::clone(&value.unwrap()), curr_ctx.clone())?;
            v = Some(
                value
                    .use_as(var_type.clone(), value.clone(), None, curr_ctx.clone())?
                    .push(),
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

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn introduce_variable_mutation(
    node: HIRArenaReference,
    curr_ctx: Option<GlobalContextKey>,
) -> DiagPossible {
    let ind = node.get_root_variable_reference_index();

    HIR_CONTEXT.with_borrow_mut(|f| {
        f.scope.mutate_entry(
            curr_ctx.unwrap(),
            |entry| {
                entry.mutate_function(
                    |ff| {
                        ff.local_context.as_mut().unwrap().variables[ind].introduce_mutation();
                    },
                    &*node,
                )
            },
            &*node,
        )
    })??;

    Ok(())
}

pub fn lower_ast_variable_assign(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::Assignment { variable, value } = node.kind.clone() {
        let variable = lower_ast_value(ASTNode::clone(&variable), curr_ctx.clone())?;

        let value = lower_ast_value(ASTNode::clone(&value), curr_ctx.clone())?;
        let value = value
            .use_as(
                variable.get_type(curr_ctx.clone())?.unwrap(),
                value.clone(),
                None,
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

        introduce_variable_mutation(variable.clone(), curr_ctx.clone())?;

        let mut n = HIRNodeKind::Assignment {
            variable: variable.clone(),
            value: value.clone(),
        };

        if let HIRNodeKind::PointerDereference(inner) = variable.kind.clone() {
            n = HIRNodeKind::PointerDerefAssign {
                pointer: inner,
                value: value.clone(),
            }
        }

        if let HIRNodeKind::FieldReference { val, name } = variable.kind.clone() {
            n = HIRNodeKind::StructFieldAssign {
                struct_val: val,
                field: name,
                value,
            }
        }

        let node = HIRNode::new(n, node.start.clone(), node.end.clone());

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, DiagResult, diags::errors::build_expected_error};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

use crate::stage2::values::lower_ast_value;

pub fn introduce_reference_ast(
    node: HIRArenaReference,
    local_ctx: Option<GlobalContextKey>,
) -> DiagPossible {
    let ind = node.get_root_variable_reference_index();

    HIR_CONTEXT.with_borrow_mut(|f| {
        f.scope.mutate_entry(
            local_ctx.unwrap(),
            |entry| {
                entry.mutate_function(
                    |ff| {
                        ff.local_context.as_mut().unwrap().variables[ind].introduce_reference();
                    },
                    &*node,
                )
            },
            &*node,
        )
    })??;

    Ok(())
}

pub fn lower_ast_pointer_reference(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::PointerReference(val) = node.kind.clone() {
        let val = lower_ast_value(ASTNode::clone(&val), local_ctx.clone())?;

        if !val.represents_pointer_referencable() {
            return Err(build_expected_error(
                &"referencable".to_string(),
                &val.get_type(local_ctx)?.unwrap(),
                &*val,
            )
            .into());
        }

        introduce_reference_ast(val.clone(), local_ctx.clone())?;

        let node = HIRNode::new(
            HIRNodeKind::PointerReference(val),
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_pointer_dereference(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::PointerDereference(val) = node.kind.clone() {
        let val = lower_ast_value(ASTNode::clone(&val), local_ctx.clone())?;

        if !val.represents_pointer_referencable() {
            return Err(build_expected_error(
                &"referencable".to_string(),
                &val.get_type(local_ctx)?.unwrap(),
                &*val,
            )
            .into());
        }

        let node = HIRNode::new(
            HIRNodeKind::PointerDereference(val),
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagResult, diags::errors::build_expected_error};
use calsc_hir::{
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};

use crate::stage2::values::lower_ast_value;

pub fn lower_ast_pointer_reference(
    node: ASTNode,
    local_ctx: Option<&LocalContext>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::PointerReference(val) = node.kind.clone() {
        let val = lower_ast_value(ASTNode::clone(&val), local_ctx)?;

        if !val.represents_pointer_referencable() {
            return Err(build_expected_error(
                &"referencable".to_string(),
                &val.get_type(local_ctx)?.unwrap(),
                &*val,
            )
            .into());
        }

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
    local_ctx: Option<&LocalContext>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::PointerDereference(val) = node.kind.clone() {
        let val = lower_ast_value(ASTNode::clone(&val), local_ctx)?;
        let val_type = val.get_type(local_ctx)?;

        if val_type.is_none() || !val_type.unwrap().is_reference() {
            return Err(build_expected_error(
                &"pointer".to_string(),
                &"non addressable type".to_string(),
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

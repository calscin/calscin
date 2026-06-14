use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{
    DiagResult,
    diags::errors::{
        build_cannot_find_element_no_closest, build_cannot_parse_error,
        build_internal_hir_node_leaked,
    },
};
use calsc_hir::{
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
use calsc_typing::{FieldHavingType, func::DeclBlockAffectedType};

use crate::stage2::{funcs::lower_ast_function_call, values::lower_ast_value};

pub fn lower_ast_lru(
    node: ASTNode,
    curr_ctx: Option<GlobalContextKey>,
) -> DiagResult<HIRArenaReference> {
    if let ASTNodeKind::StructLRUsage {
        left_expr,
        right_expr,
    } = node.kind.clone()
    {
        let left_expr = lower_ast_value(ASTNode::clone(&left_expr), curr_ctx.clone())?;
        let left_ty = left_expr.get_type(curr_ctx.clone())?;

        match &right_expr.kind {
            ASTNodeKind::FunctionCall { name, arguments: _ } => {
                if name.members.len() != 1 {
                    return Err(build_cannot_parse_error(
                        &"LRU function call".to_string(),
                        &*right_expr,
                    )
                    .into());
                }

                if !left_ty.clone().has_function(name.last().clone()) {
                    return Err(build_cannot_find_element_no_closest(&name, &node).into());
                }

                if !left_ty.is_transparent_real() {
                    return Err(build_cannot_find_element_no_closest(&name, &node).into());
                }

                let ret = lower_ast_function_call(
                    node,
                    Some(left_ty.get_transparent_real().ty),
                    curr_ctx.clone(),
                )?;

                let ret = HIRNode::clone(&ret);

                if let HIRNodeKind::FunctionCall { func, arguments } = ret.kind.clone() {
                    let mut arguments = arguments;

                    arguments.insert(0, left_expr);

                    let ret = HIRNode::new(
                        HIRNodeKind::FunctionCall { func, arguments },
                        ret.start.clone(),
                        ret.end.clone(),
                    );

                    return Ok(ret.push());
                } else {
                    return Err(build_internal_hir_node_leaked(&ret, &ret).into());
                }
            }

            ASTNodeKind::ElementReference(name) => {
                if !left_ty.has_field(name.clone()) {
                    return Err(build_cannot_find_element_no_closest(&name, &node).into());
                }

                let field_ind = left_ty.get_field_index(name.clone());

                let node = HIRNode::new(
                    HIRNodeKind::FieldReference {
                        val: left_expr,
                        field_ind,
                        name: name.clone(),
                    },
                    node.start.clone(),
                    node.end.clone(),
                );

                return Ok(node.push());
            }

            _ => return Err(build_internal_hir_node_leaked(&node, &node).into()),
        }
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

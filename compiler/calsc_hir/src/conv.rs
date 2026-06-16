//! Convertion definitions for HIR nodes

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{
        build_expected_type_error, build_internal_hir_node_leaked, build_missing_field,
        build_unexpected_type_error,
    },
};
use calsc_typing::{
    FieldHavingType, TransmutableType, base::instance::BaseTypeInstance, tree::Type,
};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{
    HIRContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};

impl HIRNode {
    /// Uses the node as a value of the given [`Type`]. Will potentially transmute the value if possible.
    ///
    /// # Error
    /// This function will error if the casting fails at any point or if the types cannot be transmuted.
    ///
    pub fn use_as(
        &self,
        ty: Type,
        curr_node: ArenaHandle,
        other_node: Option<ArenaHandle>,
        local_func_key: Option<GlobalContextKey>,
        ctx: &mut HIRContext,
    ) -> DiagResult<HIRNode> {
        if let HIRNodeKind::StructuredInit { .. } = self.kind.clone() {
            return convert_structured_init_into(self.clone(), ty, local_func_key, self, ctx);
        }

        if self.get_type(local_func_key.clone(), ctx)? == Type::Void {
            return Err(build_unexpected_type_error(&"void".to_string(), self).into());
        }

        if self.is_numerical_lit() && ty.is_direct_numeric_generic() {
            return convert_numerical_literal_into(self.clone(), ty.as_base());
        }

        let self_type: Type = self.get_type(local_func_key.clone(), ctx)?;

        if self_type == ty {
            return Ok(self.clone());
        }

        if self_type.can_transmute(ty.clone()) {
            let node = HIRNode::new(
                HIRNodeKind::CastNode {
                    original: self.clone().push(ctx),
                    into: ty,
                },
                self.start.clone(),
                self.end.clone(),
            );

            return Ok(node);
        }

        if self.is_weakly_typed(ctx) && self_type.can_transmute_weakly(ty.clone()) {
            weakly_transmute(curr_node, ty, ctx);

            return Ok(self.clone());
        }

        if other_node.is_some()
            && ctx
                .nodes
                .get(other_node.as_ref().unwrap())
                .is_weakly_typed(ctx)
            && ty.can_transmute_weakly(self_type.clone())
        {
            weakly_transmute(other_node.unwrap(), self_type.clone(), ctx);
        }

        return Err(build_expected_type_error(&self_type, &ty, self).into());
    }
}

pub fn convert_structured_init_into<K: DiagnosticSource>(
    structured_init: HIRNode,
    ty: Type,
    local_func_key: Option<GlobalContextKey>,
    origin: &K,
    ctx: &mut HIRContext,
) -> DiagResult<HIRNode> {
    if let HIRNodeKind::StructuredInit { values } = structured_init.kind {
        let mut vals = HashMap::new();

        for field in ty.get_fields() {
            if !values.contains_key(&field) {
                return Err(build_missing_field(&field, origin).into());
            }

            let field_node = ctx.nodes.get(&values[&field]).clone();

            vals.insert(
                field.clone(),
                field_node
                    .use_as(
                        ty.get_field_type(field.clone()),
                        values[&field].clone(),
                        None,
                        local_func_key.clone(),
                        ctx,
                    )?
                    .push(ctx),
            );
        }

        let node = HIRNode::new(
            HIRNodeKind::TypedStructuredInit { ty, values: vals },
            structured_init.start,
            structured_init.end,
        );

        Ok(node)
    } else {
        return Err(build_internal_hir_node_leaked(&structured_init, &structured_init).into());
    }
}

pub fn convert_numerical_literal_into(lit: HIRNode, ty: BaseTypeInstance) -> DiagResult<HIRNode> {
    let size = ty.size_specifiers[0];
    let signed = ty.ty.kind.get_signed_state();

    if let HIRNodeKind::IntLiteral(val, _, _) = &lit.kind {
        return Ok(HIRNode::new(
            HIRNodeKind::IntLiteral(*val, size, signed),
            lit.start.clone(),
            lit.end.clone(),
        ));
    }

    if let HIRNodeKind::FloatLiteral(val, _, _) = &lit.kind {
        return Ok(HIRNode::new(
            HIRNodeKind::FloatLiteral(*val, size, signed),
            lit.start.clone(),
            lit.end.clone(),
        ));
    }

    return Err(build_internal_hir_node_leaked(&lit, &lit).into());
}

pub fn weakly_transmute(curr_node: ArenaHandle, ty: Type, ctx: &mut HIRContext) {
    let node_kind = &ctx.nodes.get(&curr_node).kind.clone();

    match node_kind {
        HIRNodeKind::IntLiteral(_, _, _) => {
            let base = ty.as_base();

            if !base.ty.kind.is_int() {
                panic!()
            }

            ctx.nodes.get_mut(&curr_node).stronger_type = Some(ty);
        }

        HIRNodeKind::FloatLiteral(_, _, _) => {
            let base = ty.as_base();

            if !base.ty.kind.is_float() {
                panic!()
            }

            ctx.nodes.get_mut(&curr_node).stronger_type = Some(ty);
        }

        HIRNodeKind::MathExpression {
            left_expr,
            right_expr,
            operator: _,
        } => {
            weakly_transmute(left_expr.clone(), ty.clone(), ctx);
            weakly_transmute(right_expr.clone(), ty, ctx);
        }

        HIRNodeKind::Range {
            start,
            end,
            increment,
        } => {
            weakly_transmute(start.clone(), ty.clone(), ctx);
            weakly_transmute(end.clone(), ty.clone(), ctx);

            if increment.is_some() {
                weakly_transmute(increment.as_ref().unwrap().clone(), ty, ctx);
            }
        }

        HIRNodeKind::ArrayInit { vals } => {
            for val in vals {
                weakly_transmute(val.clone(), ty.get_inner(), ctx);
            }
        }

        #[cfg(feature = "debug")]
        kind => panic!("Unexpected {:#?}", kind),

        #[cfg(not(feature = "debug"))]
        _ => panic!("Unexpected kind"),
    }
}

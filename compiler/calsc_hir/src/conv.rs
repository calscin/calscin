//! Convertion definitions for HIR nodes

use std::{collections::HashMap, hint::unreachable_unchecked};

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_expected_type_error, build_missing_field, build_unexpected_type_error},
};
use calsc_typing::{
    FieldHavingType, TransmutableType, base::instance::BaseTypeInstance, tree::Type,
};

use crate::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
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
        curr_node: HIRArenaReference,
        other_node: Option<HIRArenaReference>,
        local_func_key: Option<GlobalContextKey>,
    ) -> DiagResult<HIRNode> {
        if let HIRNodeKind::StructuredInit { .. } = self.kind.clone() {
            return convert_structured_init_into(self.clone(), ty, local_func_key, self);
        }

        if self.get_type(local_func_key.clone())? == Type::Void {
            return Err(build_unexpected_type_error(&"void".to_string(), self).into());
        }

        if self.is_numerical_lit() && ty.is_direct_numeric_generic() {
            return convert_numerical_literal_into(self.clone(), ty.as_base());
        }

        let self_type: Type = self.get_type(local_func_key.clone())?;

        if self_type == ty {
            return Ok(self.clone());
        }

        if self_type.can_transmute(ty.clone()) {
            let node = HIRNode::new(
                HIRNodeKind::CastNode {
                    original: self.clone().push(),
                    into: ty,
                },
                self.start.clone(),
                self.end.clone(),
            );

            return Ok(node);
        }

        if self.is_weakly_typed() && self_type.can_transmute_weakly(ty.clone()) {
            weakly_transmute(curr_node, ty);

            return Ok(self.clone());
        }

        if other_node.is_some()
            && other_node.as_ref().unwrap().is_weakly_typed()
            && ty.can_transmute_weakly(self_type.clone())
        {
            weakly_transmute(other_node.unwrap(), self_type.clone());
        }

        return Err(build_expected_type_error(&self_type, &ty, self).into());
    }
}

pub fn convert_structured_init_into<K: DiagnosticSource>(
    structured_init: HIRNode,
    ty: Type,
    local_func_key: Option<GlobalContextKey>,
    origin: &K,
) -> DiagResult<HIRNode> {
    if let HIRNodeKind::StructuredInit { values } = structured_init.kind {
        let mut vals = HashMap::new();

        for field in ty.get_fields() {
            if !values.contains_key(&field) {
                return Err(build_missing_field(&field, origin).into());
            }

            vals.insert(
                field.clone(),
                values[&field]
                    .use_as(
                        ty.get_field_type(field.clone()),
                        values[&field].clone(),
                        None,
                        local_func_key.clone(),
                    )?
                    .push(),
            );
        }

        let node = HIRNode::new(
            HIRNodeKind::TypedStructuredInit { ty, values: vals },
            structured_init.start,
            structured_init.end,
        );

        Ok(node)
    } else {
        unsafe { unreachable_unchecked() }
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

    unsafe { unreachable_unchecked() }
}

pub fn weakly_transmute(curr_node: HIRArenaReference, ty: Type) {
    match &curr_node.kind {
        HIRNodeKind::IntLiteral(_, _, _) => {
            let base = ty.as_base();

            if !base.ty.kind.is_int() {
                panic!()
            }

            HIR_CONTEXT
                .with(|f| f.borrow_mut().nodes.arena[curr_node.refer].stronger_type = Some(ty));
        }

        HIRNodeKind::FloatLiteral(_, _, _) => {
            let base = ty.as_base();

            if !base.ty.kind.is_float() {
                panic!()
            }

            HIR_CONTEXT
                .with(|f| f.borrow_mut().nodes.arena[curr_node.refer].stronger_type = Some(ty));
        }

        HIRNodeKind::MathExpression {
            left_expr,
            right_expr,
            operator: _,
        } => {
            weakly_transmute(left_expr.clone(), ty.clone());
            weakly_transmute(right_expr.clone(), ty);
        }

        HIRNodeKind::Range {
            start,
            end,
            increment,
        } => {
            weakly_transmute(start.clone(), ty.clone());
            weakly_transmute(end.clone(), ty.clone());

            if increment.is_some() {
                weakly_transmute(increment.as_ref().unwrap().clone(), ty);
            }
        }

        HIRNodeKind::ArrayInit { vals } => {
            for val in vals {
                weakly_transmute(val.clone(), ty.get_inner());
            }
        }

        #[cfg(feature = "debug")]
        kind => panic!("Unexpected {:#?}", kind),

        #[cfg(not(feature = "debug"))]
        _ => panic!("Unexpected kind"),
    }
}

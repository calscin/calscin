//! Convertion definitions for HIR nodes

use std::{collections::HashMap, hint::unreachable_unchecked};

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_expected_error, build_missing_field, build_unexpected_error},
};
use calsc_typing::{
    FieldHavingType, TransmutableType, base::instance::BaseTypeInstance, tree::Type,
};

use crate::{
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
};

impl HIRNode {
    /// Uses the node as a value of the given [`Type`]. Will potentially transmute the value if possible.
    ///
    /// # Error
    /// This function will error if the casting fails at any point or if the types cannot be transmuted.
    ///
    pub fn use_as(&self, ty: Type, local_ctx: Option<&LocalContext>) -> DiagResult<HIRNode> {
        if self.get_type(local_ctx)?.is_none() {
            return Err(build_unexpected_error(&"void".to_string(), self).into());
        }

        let self_type = unsafe { self.get_type(local_ctx.clone())?.unwrap_unchecked() };

        if self_type == ty {
            return Ok(self.clone());
        }

        if let HIRNodeKind::StructuredInit { .. } = self.kind.clone() {
            return convert_structured_init_into(self.clone(), ty, local_ctx, self);
        }

        if !self_type.can_transmute(ty.clone()) {
            return Err(build_expected_error(&ty, &self_type, self).into());
        }

        if self.is_numerical_lit() && ty.is_direct_numeric_generic() {
            return convert_numerical_literal_into(self.clone(), ty.as_base());
        }

        let node = HIRNode::new(
            HIRNodeKind::CastNode {
                original: self.clone().push(),
                into: ty,
            },
            self.start.clone(),
            self.end.clone(),
        );

        Ok(node)
    }
}

pub fn convert_structured_init_into<K: DiagnosticSource>(
    structured_init: HIRNode,
    ty: Type,
    local_ctx: Option<&LocalContext>,
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
                    .use_as(ty.get_field_type(field.clone()), local_ctx)?
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

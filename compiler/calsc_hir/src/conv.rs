//! Convertion definitions for HIR nodes

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{
        build_expected_type_error, build_internal_hir_node_leaked, build_missing_field,
        build_type_cast_failed_no_from, build_unexpected_type_error,
    },
};

use calsc_typing::{
    ctx::TypeCtx,
    into::TypeTransmutation,
    traits::FieldedType,
    types::{HeldPrimitive, TypeKind},
};
use calsc_utils::{alloc::arena::ArenaHandle, display_with_to_string};

use crate::{
    HIRContext,
    file::HIRFileContext,
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
        ty: &TypeKind,
        curr_node: ArenaHandle,
        other_node: Option<ArenaHandle>,
        local_func_key: Option<GlobalContextKey>,
        ctx: &mut HIRContext,
        file_ctx: &HIRFileContext,
    ) -> DiagResult<HIRNode> {
        if let HIRNodeKind::StructuredInit { .. } = self.kind.clone() {
            return convert_structured_init_into(
                self.clone(),
                ty,
                local_func_key,
                self,
                ctx,
                file_ctx,
            );
        }

        if self.get_type(local_func_key.clone(), ctx, Some(file_ctx))? == TypeKind::Void {
            return Err(build_unexpected_type_error(&"void".to_string(), self).into());
        }

        if self.is_numerical_lit() && ty.is_directly_numeric() {
            return convert_numerical_literal_into(self.clone(), ty.as_primitive(), &ctx.type_ctx);
        }

        let self_type: TypeKind = self.get_type(local_func_key.clone(), ctx, Some(file_ctx))?;

        if &self_type == ty {
            return Ok(self.clone());
        }

        if self_type.can_transmute(ty, &ctx.type_ctx) {
            let node = HIRNode::new(
                HIRNodeKind::CastNode {
                    original: self.clone().push(ctx),
                    into: ty.clone(),
                    explicit_cast: false,
                },
                self.start.clone(),
                self.end.clone(),
            );

            return Ok(node);
        }

        if self.is_weakly_typed(ctx) && self_type.can_transmute_weakly(ty, &ctx.type_ctx) {
            weakly_transmute(curr_node, ty, ctx);

            return Ok(self.clone());
        }

        if other_node.is_some()
            && ctx
                .nodes
                .get(other_node.as_ref().unwrap())
                .is_weakly_typed(ctx)
            && ty.can_transmute_weakly(&self_type, &ctx.type_ctx)
        {
            weakly_transmute(other_node.unwrap(), &self_type, ctx);
        }

        return Err(build_expected_type_error(
            &display_with_to_string(ty, &ctx.type_ctx),
            &display_with_to_string(&self_type, &ctx.type_ctx),
            self,
        )
        .into());
    }
}

pub fn convert_structured_init_into<K: DiagnosticSource>(
    structured_init: HIRNode,
    ty: &TypeKind,
    local_func_key: Option<GlobalContextKey>,
    origin: &K,
    ctx: &mut HIRContext,
    file_ctx: &HIRFileContext,
) -> DiagResult<HIRNode> {
    if let HIRNodeKind::StructuredInit { values } = structured_init.kind {
        let mut vals = HashMap::new();

        for field in ty.get_fields(&ctx.type_ctx) {
            if !values.contains_key(&field) {
                return Err(build_missing_field(&field, origin).into());
            }

            let field_node = ctx.nodes.get(&values[&field]).clone();

            vals.insert(
                field.clone(),
                field_node
                    .use_as(
                        &ty.get_field_safe(&field, &ctx.type_ctx, origin)?,
                        values[&field].clone(),
                        None,
                        local_func_key.clone(),
                        ctx,
                        file_ctx,
                    )?
                    .push(ctx),
            );
        }

        let node = HIRNode::new(
            HIRNodeKind::TypedStructuredInit {
                ty: ty.clone(),
                values: vals,
            },
            structured_init.start,
            structured_init.end,
        );

        Ok(node)
    } else {
        return Err(build_internal_hir_node_leaked(&structured_init, &structured_init).into());
    }
}

pub fn convert_numerical_literal_into(
    lit: HIRNode,
    ty: HeldPrimitive,
    ctx: &TypeCtx,
) -> DiagResult<HIRNode> {
    println!("{:#?}", ty);

    assert!(ty.size.is_active() || ty.ty.is_size());

    let size = ty.size.0;
    let ty = ty.ty.clone();
    let signed = ty.get_signed_state();

    let kind = match &lit.kind {
        HIRNodeKind::IntLiteral(val, _, _) => {
            if ty.is_float() {
                HIRNodeKind::FloatLiteral(*val as f64, size, signed)
            } else if ty.is_int() {
                HIRNodeKind::IntLiteral(*val, size, signed)
            } else if ty.is_size() {
                HIRNodeKind::IntLiteral(*val, usize::BITS as usize, false)
            } else {
                return Err(build_type_cast_failed_no_from(
                    &display_with_to_string(&ty, ctx),
                    &lit,
                )
                .into());
            }
        }

        HIRNodeKind::FloatLiteral(val, _, _) => {
            if ty.is_int() {
                HIRNodeKind::IntLiteral(*val as i128, size, signed)
            } else if ty.is_float() {
                HIRNodeKind::FloatLiteral(*val, size, signed)
            } else {
                return Err(build_type_cast_failed_no_from(
                    &display_with_to_string(&ty, ctx),
                    &lit,
                )
                .into());
            }
        }

        _ => return Err(build_internal_hir_node_leaked(&lit, &lit).into()),
    };

    Ok(HIRNode::new(kind, lit.start.clone(), lit.end.clone()))
}

pub fn weakly_transmute(curr_node: ArenaHandle, ty: &TypeKind, ctx: &mut HIRContext) {
    let node_kind = &ctx.nodes.get(&curr_node).kind.clone();

    match node_kind {
        HIRNodeKind::IntLiteral(_, _, _) => {
            let base = ty.as_primitive();

            if !base.ty.is_int() {
                panic!()
            }

            ctx.nodes.get_mut(&curr_node).stronger_type = Some(ty.clone());
        }

        HIRNodeKind::FloatLiteral(_, _, _) => {
            let base = ty.as_primitive();

            if !base.ty.is_float() {
                panic!()
            }

            ctx.nodes.get_mut(&curr_node).stronger_type = Some(ty.clone());
        }

        HIRNodeKind::MathExpression {
            left_expr,
            right_expr,
            operator: _,
        } => {
            weakly_transmute(left_expr.clone(), ty, ctx);
            weakly_transmute(right_expr.clone(), ty, ctx);
        }

        HIRNodeKind::Range {
            start,
            end,
            increment,
        } => {
            weakly_transmute(start.clone(), ty, ctx);
            weakly_transmute(end.clone(), ty, ctx);

            if increment.is_some() {
                weakly_transmute(increment.as_ref().unwrap().clone(), ty, ctx);
            }
        }

        HIRNodeKind::ArrayInit { vals } => {
            for val in vals {
                let inner = ty.get_inner(&ctx.type_ctx).clone();

                weakly_transmute(val.clone(), &inner, ctx);
            }
        }

        #[cfg(feature = "debug")]
        kind => panic!("Unexpected {:#?}", kind),

        #[cfg(not(feature = "debug"))]
        _ => panic!("Unexpected kind"),
    }
}

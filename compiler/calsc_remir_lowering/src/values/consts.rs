use calsc_diagnostics::{
    DiagResult,
    diags::errors::{
        InternalErrors, build_internal_hir_node_leaked, build_internal_singleton_error,
    },
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_typing_v2::traits::FieldedType;
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    builders::{
        build_const_array, build_const_float, build_const_int, build_const_string,
        build_const_struct,
    },
    module::Module,
    values::BaseSSAValue,
};

use crate::{result::CalscinRemirResult, types::lower_type, values::lower_hir_value};

pub fn lower_hir_literal(
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &mut HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = hirctx.nodes.get(&node).clone();

    let master_type = node_ref.stronger_type.clone();

    match node_ref.kind.clone() {
        HIRNodeKind::IntLiteral(value, size, signed) => {
            let mut size = size;
            let mut signed = signed;

            if master_type.is_some() {
                let master_type = master_type.unwrap().as_primitive();

                if !master_type.0.is_int() {
                    return Err(build_internal_singleton_error(
                        InternalErrors::StrongerTypeLiterals,
                        &node_ref,
                    )
                    .into());
                }

                size = master_type.1.0;
                signed = master_type.0.get_signed_state();
            }

            let val = build_const_int(module, value, size, signed)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            Ok(val.into())
        }

        HIRNodeKind::FloatLiteral(value, size, _) => {
            let mut size = size;

            if master_type.is_some() {
                let master_type = master_type.unwrap().as_primitive();

                if !master_type.0.is_float() {
                    return Err(build_internal_singleton_error(
                        InternalErrors::StrongerTypeLiterals,
                        &node_ref,
                    )
                    .into());
                }

                size = master_type.1.0;
            }

            let val = build_const_float(module, value, size)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            Ok(val.into())
        }

        HIRNodeKind::BooleanLiteral(v) => {
            let mut value = 0;

            if v {
                value = 1;
            }

            let val = build_const_int(module, value, 1, false)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            Ok(val.into())
        }

        HIRNodeKind::StringLiteral(val) => {
            let val = build_const_string(module, val).unwrap();

            Ok(val.into())
        }

        HIRNodeKind::TypedStructuredInit { ty, values } => {
            let mut vals = vec![];
            let mir_ty = lower_type(ty.clone(), &hirctx.type_ctx)?;

            for field in ty.get_fields(&hirctx.type_ctx) {
                vals.push(lower_hir_value(
                    values[&field].clone(),
                    ctx,
                    module,
                    hirctx,
                )?)
            }

            let val = build_const_struct(module, mir_ty, vals)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            Ok(val.into())
        }

        _ => return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into()),
    }
}

pub fn lower_hir_array_const(
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &mut HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = hirctx.nodes.get(&node).clone();

    if let HIRNodeKind::ArrayInit { vals } = node_ref.kind.clone() {
        let mut mir_vals = vec![]; // This cannot use iter.map due to the ? operator

        for val in vals {
            mir_vals.push(lower_hir_value(val, ctx, module, hirctx)?);
        }

        let val = build_const_array(module, mir_vals)
            .convert(node_ref.start.clone(), node_ref.end.clone())?;

        Ok(val.into())
    } else {
        return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into());
    }
}

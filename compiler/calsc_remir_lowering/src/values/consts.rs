use calsc_diagnostics::{
    DiagResult,
    diags::errors::{
        InternalErrors, build_internal_hir_node_leaked, build_internal_singleton_error,
    },
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use calsc_typing::FieldHavingType;
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
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    let master_type = node.stronger_type.clone();

    match node.kind.clone() {
        HIRNodeKind::IntLiteral(value, size, signed) => {
            let mut size = size;
            let mut signed = signed;

            if master_type.is_some() {
                let master_type = master_type.unwrap().as_base();

                if !master_type.ty.kind.is_int() {
                    return Err(build_internal_singleton_error(
                        InternalErrors::StrongerTypeLiterals,
                        &*node,
                    )
                    .into());
                }

                size = master_type.size_specifiers[0];
                signed = master_type.ty.kind.get_signed_state();
            }

            let val = build_const_int(module, value, size, signed)
                .convert(node.start.clone(), node.end.clone())?;

            Ok(val.into())
        }

        HIRNodeKind::FloatLiteral(value, size, _) => {
            let mut size = size;

            if master_type.is_some() {
                let master_type = master_type.unwrap().as_base();

                if !master_type.ty.kind.is_float() {
                    return Err(build_internal_singleton_error(
                        InternalErrors::StrongerTypeLiterals,
                        &*node,
                    )
                    .into());
                }

                size = master_type.size_specifiers[0];
            }

            let val = build_const_float(module, value, size)
                .convert(node.start.clone(), node.end.clone())?;

            Ok(val.into())
        }

        HIRNodeKind::BooleanLiteral(v) => {
            let mut value = 0;

            if v {
                value = 1;
            }

            let val = build_const_int(module, value, 1, false)
                .convert(node.start.clone(), node.end.clone())?;

            Ok(val.into())
        }

        HIRNodeKind::StringLiteral(val) => {
            let val = build_const_string(module, val).unwrap();

            Ok(val.into())
        }

        HIRNodeKind::TypedStructuredInit { ty, values } => {
            let mut vals = vec![];
            let mir_ty = lower_type(ty.clone())?;

            for field in ty.get_fields() {
                vals.push(lower_hir_value(
                    values[&field].clone(),
                    ctx,
                    module,
                    hirctx,
                )?)
            }

            let val = build_const_struct(module, mir_ty, vals)
                .convert(node.start.clone(), node.end.clone())?;

            Ok(val.into())
        }

        _ => return Err(build_internal_hir_node_leaked(&node, &*node).into()),
    }
}

pub fn lower_hir_array_const(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    if let HIRNodeKind::ArrayInit { vals } = node.kind.clone() {
        let mut mir_vals = vec![]; // This cannot use iter.map due to the ? operator

        for val in vals {
            mir_vals.push(lower_hir_value(val, ctx, module, hirctx)?);
        }

        let val =
            build_const_array(module, mir_vals).convert(node.start.clone(), node.end.clone())?;

        Ok(val.into())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &*node).into());
    }
}

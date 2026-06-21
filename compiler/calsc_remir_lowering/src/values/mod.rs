use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_expected_type_error, build_internal_hir_node_leaked},
};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    builders::{build_extract_value, build_load, build_struct_gep},
    module::Module,
    values::{BaseSSAValue, ValueType, ptr::SSAPointerValue, structs::SSAStructValue},
};

use crate::{
    funcs::lower_hir_function_call,
    indexes::lower_hir_index_usage,
    result::CalscinRemirResult,
    values::{
        bool::{lower_hir_compare, lower_hir_inverse_condition},
        casts::lower_hir_cast_node,
        consts::{lower_hir_array_const, lower_hir_literal},
        math::lower_hir_math_operation,
        ptrs::{lower_hir_pointer_dereference, lower_hir_pointer_reference},
    },
    vars::lower_hir_variable_reference_val,
};

pub mod bool;
pub mod casts;
pub mod consts;
pub mod math;
pub mod ptrs;

pub fn lower_hir_value(
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = hirctx.nodes.get(&node);

    match &node_ref.kind {
        HIRNodeKind::IntLiteral(_, _, _)
        | HIRNodeKind::FloatLiteral(_, _, _)
        | HIRNodeKind::StringLiteral(_)
        | HIRNodeKind::BooleanLiteral(_)
        | HIRNodeKind::TypedStructuredInit { .. } => lower_hir_literal(node, ctx, module, hirctx),

        HIRNodeKind::InverseCondition(_) => {
            Ok(lower_hir_inverse_condition(node, ctx, module, hirctx)?.into())
        }

        HIRNodeKind::PointerReference(_) => lower_hir_pointer_reference(node, ctx, module, hirctx),
        HIRNodeKind::PointerDereference(_) => {
            lower_hir_pointer_dereference(node, ctx, module, hirctx)
        }

        HIRNodeKind::MathExpression { .. } => lower_hir_math_operation(node, ctx, module, hirctx),
        HIRNodeKind::CompareExpression { .. } => {
            Ok(lower_hir_compare(node, ctx, module, hirctx)?.into())
        }

        HIRNodeKind::FunctionCall { .. } => {
            let val = lower_hir_function_call(node.clone(), ctx, module, hirctx)?;

            if val.is_some() {
                Ok(val.unwrap())
            } else {
                Err(build_expected_type_error(
                    &"void".to_string(),
                    &"non-void".to_string(),
                    node_ref,
                )
                .into())
            }
        }

        HIRNodeKind::VariableReference { .. } => {
            lower_hir_variable_reference_val(node, ctx, module, hirctx)
        }

        HIRNodeKind::FieldReference { .. } => lower_hir_field_reference(node, ctx, module, hirctx),

        HIRNodeKind::IndexUsage { .. } => lower_hir_index_usage(node, ctx, module, hirctx),

        HIRNodeKind::ArrayInit { .. } => lower_hir_array_const(node, ctx, module, hirctx),

        HIRNodeKind::CastNode { .. } => lower_hir_cast_node(node, ctx, module, hirctx),

        _ => return Err(build_internal_hir_node_leaked(&node, node_ref).into()),
    }
}

pub fn lower_hir_field_reference(
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = hirctx.nodes.get(&node);

    if let HIRNodeKind::FieldReference {
        val,
        field_ind,
        name: _,
    } = node_ref.kind.clone()
    {
        let val = lower_hir_value(val, ctx, module, hirctx)?;

        let field_val;

        if let ValueType::Pointer(_) = &val.value_type {
            let val: SSAPointerValue = val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
            let ptr = build_struct_gep(module, val, field_ind)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            field_val =
                build_load(module, ptr).convert(node_ref.start.clone(), node_ref.end.clone())?;
        } else {
            let val: SSAStructValue = val
                .try_into()
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            field_val = build_extract_value(module, val, field_ind)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
        }

        Ok(field_val)
    } else {
        return Err(build_internal_hir_node_leaked(node_ref, node_ref).into());
    }
}

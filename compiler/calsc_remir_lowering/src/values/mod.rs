use std::hint::unreachable_unchecked;

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_hir::{
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
    refs::HIRArenaReference,
};
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
        consts::lower_hir_literal,
        math::lower_hir_math_operation,
        ptrs::{lower_hir_pointer_dereference, lower_hir_pointer_reference},
    },
    vars::lower_hir_variable_reference_val,
};

pub mod bool;
pub mod consts;
pub mod math;
pub mod ptrs;

pub fn lower_hir_value(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    match &node.kind {
        HIRNodeKind::IntLiteral(_, _, _)
        | HIRNodeKind::FloatLiteral(_, _, _)
        | HIRNodeKind::StringLiteral(_)
        | HIRNodeKind::BooleanLiteral(_)
        | HIRNodeKind::TypedStructuredInit { .. } => lower_hir_literal(node, ctx, module),

        HIRNodeKind::InverseCondition(_) => {
            Ok(lower_hir_inverse_condition(node, ctx, module)?.into())
        }

        HIRNodeKind::PointerReference(_) => lower_hir_pointer_reference(node, ctx, module),
        HIRNodeKind::PointerDereference(_) => lower_hir_pointer_dereference(node, ctx, module),

        HIRNodeKind::MathExpression { .. } => lower_hir_math_operation(node, ctx, module),
        HIRNodeKind::CompareExpression { .. } => Ok(lower_hir_compare(node, ctx, module)?.into()),

        HIRNodeKind::FunctionCall { .. } => {
            let val = lower_hir_function_call(node.clone(), ctx, module)?;

            if val.is_some() {
                Ok(val.unwrap())
            } else {
                Err(build_unexpected_error(&"void returning function".to_string(), &*node).into())
            }
        }

        HIRNodeKind::VariableReference { .. } => {
            lower_hir_variable_reference_val(node, ctx, module)
        }

        HIRNodeKind::FieldReference { .. } => lower_hir_field_reference(node, ctx, module),

        HIRNodeKind::IndexUsage { .. } => lower_hir_index_usage(node, ctx, module),

        e => panic!("Unexpected kind {:#?}", e),
    }
}

pub fn lower_hir_field_reference(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    if let HIRNodeKind::FieldReference {
        val,
        field_ind,
        name: _,
    } = node.kind.clone()
    {
        let val = lower_hir_value(val, ctx, module)?;

        let field_val;

        if let ValueType::Pointer(_) = &val.value_type {
            let val: SSAPointerValue = val
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;
            let ptr = build_struct_gep(module, val, field_ind)
                .convert(node.start.clone(), node.end.clone())?;

            field_val = build_load(module, ptr).convert(node.start.clone(), node.end.clone())?;
        } else {
            let val: SSAStructValue = val
                .try_into()
                .convert(node.start.clone(), node.end.clone())?;

            field_val = build_extract_value(module, val, field_ind)
                .convert(node.start.clone(), node.end.clone())?;
        }

        Ok(field_val)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

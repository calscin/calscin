use std::hint::unreachable_unchecked;

use calsc_diagnostics::DiagResult;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use calsc_utils::cmp::{CompareOperator, ComparePredicate};
use remir::{
    builders::{build_float_compare, build_int_compare},
    module::Module,
    values::{BaseSSAValue, ValueType, float::SSAFloatValue, int::SSAIntValue},
};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

pub fn convert_compare_operator(operation: CompareOperator) -> remir::misc::CompareOperator {
    match operation.predicate {
        ComparePredicate::Equal => remir::misc::CompareOperator::Eq,
        ComparePredicate::GreaterThan => {
            if operation.also_equal {
                remir::misc::CompareOperator::Ge
            } else {
                remir::misc::CompareOperator::Gt
            }
        }

        ComparePredicate::LowerThan => {
            if operation.also_equal {
                remir::misc::CompareOperator::Le
            } else {
                remir::misc::CompareOperator::Lt
            }
        }

        ComparePredicate::NotEqual => remir::misc::CompareOperator::Ne,
    }
}

pub fn lower_hir_compare(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<SSAIntValue> {
    if let HIRNodeKind::CompareExpression {
        left_expr,
        right_expr,
        operator,
    } = node.kind.clone()
    {
        let left_expr_val = lower_hir_value(left_expr, ctx, module)?;
        let right_expr_val = lower_hir_value(right_expr, ctx, module)?;

        let res: SSAIntValue;

        if let ValueType::Int(signed, _) = left_expr_val.value_type.clone() {
            let left_expr_val = SSAIntValue::try_from(left_expr_val)
                .convert(node.start.clone(), node.end.clone())?;
            let right_expr_val = SSAIntValue::try_from(right_expr_val)
                .convert(node.start.clone(), node.end.clone())?;

            res = build_int_compare(
                module,
                left_expr_val,
                right_expr_val,
                convert_compare_operator(operator),
                signed,
            )
            .convert(node.start.clone(), node.end.clone())?;
        } else {
            let left_expr_val = SSAFloatValue::try_from(left_expr_val)
                .convert(node.start.clone(), node.end.clone())?;
            let right_expr_val = SSAFloatValue::try_from(right_expr_val)
                .convert(node.start.clone(), node.end.clone())?;

            //res = build_float_compare(
            //    module,
            //    left_expr_val,
            //    right_expr_val,
            //    convert_compare_operator(operator),
            //)
            //.convert(node.start.clone(), node.end.clone())?;

            todo!()
        }

        Ok(res)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

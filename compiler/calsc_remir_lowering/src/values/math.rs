use std::hint::unreachable_unchecked;

use calsc_diagnostics::DiagResult;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use calsc_utils::math::MathOperation;
use remir::{
    builders::{build_math_op_float, build_math_op_int},
    module::Module,
    values::{BaseSSAValue, float::SSAFloatValue, int::SSAIntValue},
};

use crate::{result::CalscinRemirResult, values::lower_hir_value, writes::lower_hir_writable};

pub fn convert_math_operator(math: MathOperation) -> DiagResult<remir::misc::MathOperator> {
    Ok(match math {
        MathOperation::And => remir::misc::MathOperator::And,
        MathOperation::Add => remir::misc::MathOperator::Add,
        MathOperation::Div => remir::misc::MathOperator::Div,
        MathOperation::Mod => remir::misc::MathOperator::Mod,
        MathOperation::Mul => remir::misc::MathOperator::Mul,
        MathOperation::Nor => remir::misc::MathOperator::Nor,
        MathOperation::Or => remir::misc::MathOperator::Or,
        MathOperation::ShiftLeft => remir::misc::MathOperator::Shl,
        MathOperation::ShiftRight => remir::misc::MathOperator::Shr,
        MathOperation::Sub => remir::misc::MathOperator::Sub,
        MathOperation::Xor => remir::misc::MathOperator::Xor,
    })
}

pub fn lower_hir_math_operation(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    if let HIRNodeKind::MathExpression {
        left_expr,
        right_expr,
        operator,
    } = node.kind.clone()
    {
        let ty = node.get_type(Some(ctx.local_key.clone()))?.unwrap();
        let left_expr_val = lower_hir_value(left_expr.clone(), ctx, module)?;
        let right_expr_val = lower_hir_value(right_expr, ctx, module)?;

        let res: BaseSSAValue;

        if ty.is_base() && ty.as_base().ty.kind.is_int() {
            let left_expr = SSAIntValue::try_from(left_expr_val)
                .convert(node.start.clone(), node.end.clone())?;
            let right_expr = SSAIntValue::try_from(right_expr_val)
                .convert(node.start.clone(), node.end.clone())?;

            res = build_math_op_int(
                module,
                left_expr,
                right_expr,
                convert_math_operator(operator.operation.clone())?,
                ty.as_base().ty.kind.get_signed_state(),
                !operator.fast,
                !operator.fast,
                operator.fast,
            )
            .convert(node.start.clone(), node.end.clone())?
            .into()
        } else {
            let left_expr = SSAFloatValue::try_from(left_expr_val)
                .convert(node.start.clone(), node.end.clone())?;

            let right_expr = SSAFloatValue::try_from(right_expr_val)
                .convert(node.start.clone(), node.end.clone())?;

            res = build_math_op_float(
                module,
                left_expr,
                right_expr,
                convert_math_operator(operator.operation.clone())?,
                !operator.fast,
                !operator.fast,
                operator.fast,
            )
            .convert(node.start.clone(), node.end.clone())?
            .into()
        }

        if operator.assigns {
            lower_hir_writable(left_expr, ctx, module, res.clone())?;
        }

        Ok(res)
    } else {
        unsafe { unreachable_unchecked() }
    }
}

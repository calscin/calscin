use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{HIRContext, localctx::LocalContext, nodes::HIRNodeKind};
use calsc_utils::{alloc::arena::ArenaHandle, math::MathOperation};
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
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &mut HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node_ref = hirctx.nodes.get(&node).clone();

    if let HIRNodeKind::MathExpression {
        left_expr,
        right_expr,
        operator,
    } = node_ref.kind.clone()
    {
        let left_expr_ref = hirctx.nodes.get(&left_expr).clone();

        let ty = left_expr_ref.get_type(Some(ctx.local_key.clone()), hirctx, None)?;

        let left_expr_val = lower_hir_value(left_expr.clone(), ctx, module, hirctx)?;
        let right_expr_val = lower_hir_value(right_expr, ctx, module, hirctx)?;

        let res: BaseSSAValue;

        if ty.is_directly_primitive()
            && (ty.as_primitive().ty.is_int() || ty.as_primitive().ty.is_size())
        {
            let left_expr = SSAIntValue::try_from(left_expr_val)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;
            let right_expr = SSAIntValue::try_from(right_expr_val)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            res = build_math_op_int(
                module,
                left_expr,
                right_expr,
                convert_math_operator(operator.operation.clone())?,
                ty.as_primitive().ty.get_signed_state(),
                !operator.fast,
                !operator.fast,
                operator.fast,
            )
            .convert(node_ref.start.clone(), node_ref.end.clone())?
            .into()
        } else {
            let left_expr = SSAFloatValue::try_from(left_expr_val)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            let right_expr = SSAFloatValue::try_from(right_expr_val)
                .convert(node_ref.start.clone(), node_ref.end.clone())?;

            res = build_math_op_float(
                module,
                left_expr,
                right_expr,
                convert_math_operator(operator.operation.clone())?,
                !operator.fast,
                !operator.fast,
                operator.fast,
            )
            .convert(node_ref.start.clone(), node_ref.end.clone())?
            .into()
        }

        if operator.assigns {
            lower_hir_writable(left_expr, ctx, module, res.clone(), hirctx)?;
        }

        Ok(res)
    } else {
        return Err(build_internal_hir_node_leaked(&node_ref, &node_ref).into());
    }
}

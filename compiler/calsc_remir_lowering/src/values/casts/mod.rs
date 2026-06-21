use calsc_diagnostics::{DiagResult, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{
    HIRContext,
    localctx::LocalContext,
    nodes::{HIRNode, HIRNodeKind},
};
use calsc_utils::alloc::arena::ArenaHandle;
use remir::{
    builders::{
        build_bit_cast, build_float_change_size, build_float_to_int, build_int_change_size,
        build_int_to_float,
    },
    module::Module,
    values::{BaseSSAValue, ValueType, float::SSAFloatValue, int::SSAIntValue},
};

use crate::{result::CalscinRemirResult, types::lower_type, values::lower_hir_value};

pub fn lower_hir_cast_node(
    node: ArenaHandle,
    ctx: &LocalContext,
    module: &mut Module,
    hirctx: &HIRContext,
) -> DiagResult<BaseSSAValue> {
    let node = hirctx.nodes.get(&node).clone();

    if let HIRNodeKind::CastNode {
        original,
        into,
        explicit_cast: _,
    } = node.kind.clone()
    {
        let val = lower_hir_value(original, ctx, module, hirctx)?;
        let into = lower_type(into)?;

        lower_hir_cast(val, into, module, &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_hir_cast(
    val: BaseSSAValue,
    into: ValueType,
    module: &mut Module,
    node: &HIRNode,
) -> DiagResult<BaseSSAValue> {
    if let ValueType::Int(_, _) = val.value_type.clone() {
        let val =
            SSAIntValue::try_from(val.clone()).convert(node.start.clone(), node.end.clone())?;

        match into {
            ValueType::Float(_) => {
                return Ok(build_int_to_float(module, val, into)
                    .convert(node.start.clone(), node.end.clone())?
                    .into());
            }

            ValueType::Int(_, _) => {
                return Ok(build_int_change_size(module, val, into)
                    .convert(node.start.clone(), node.end.clone())?
                    .into());
            }

            _ => {}
        };
    }

    if let ValueType::Float(_) = val.value_type.clone() {
        let val =
            SSAFloatValue::try_from(val.clone()).convert(node.start.clone(), node.end.clone())?;

        match into {
            ValueType::Float(_) => {
                return Ok(build_float_change_size(module, val, into)
                    .convert(node.start.clone(), node.end.clone())?
                    .into());
            }

            ValueType::Int(_, _) => {
                return Ok(build_float_to_int(module, val, into)
                    .convert(node.start.clone(), node.end.clone())?
                    .into());
            }

            _ => {}
        }
    }

    // Default handling for unhandled casts, we just bitcast

    build_bit_cast(module, val, into).convert(node.start.clone(), node.end.clone())
}

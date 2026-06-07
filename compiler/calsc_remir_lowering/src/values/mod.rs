use calsc_diagnostics::DiagResult;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use remir::{module::Module, values::BaseSSAValue};

use crate::{
    values::{
        bool::lower_hir_compare,
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

        HIRNodeKind::PointerReference(_) => lower_hir_pointer_reference(node, ctx, module),
        HIRNodeKind::PointerDereference(_) => lower_hir_pointer_dereference(node, ctx, module),

        HIRNodeKind::MathExpression { .. } => lower_hir_math_operation(node, ctx, module),
        HIRNodeKind::CompareExpression { .. } => Ok(lower_hir_compare(node, ctx, module)?.into()),

        HIRNodeKind::VariableReference { .. } => {
            lower_hir_variable_reference_val(node, ctx, module)
        }

        kind => panic!("Unexpected {:#?}", kind),
    }
}

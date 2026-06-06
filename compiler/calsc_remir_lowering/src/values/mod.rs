use calsc_diagnostics::DiagResult;
use calsc_hir::{
    HIRContext,
    nodes::{HIRNode, HIRNodeKind},
};
use remir::{module::Module, values::BaseSSAValue};

use crate::values::consts::lower_hir_literal;

pub mod consts;

pub fn lower_hir_value(
    node: HIRNode,
    ctx: &HIRContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    match node.kind {
        HIRNodeKind::IntLiteral(_, _, _)
        | HIRNodeKind::FloatLiteral(_, _, _)
        | HIRNodeKind::StringLiteral(_)
        | HIRNodeKind::BooleanLiteral(_)
        | HIRNodeKind::TypedStructuredInit { .. } => lower_hir_literal(node, ctx, module),

        kind => panic!("Unexpected {:#?}", kind),
    }
}

use calsc_diagnostics::DiagResult;
use calsc_hir::{localctx::LocalContext, nodes::HIRNodeKind, refs::HIRArenaReference};
use calsc_typing::FieldHavingType;
use remir::{
    builders::{build_const_float, build_const_int, build_const_string},
    module::Module,
    values::BaseSSAValue,
};

use crate::{result::CalscinRemirResult, values::lower_hir_value};

pub fn lower_hir_literal(
    node: HIRArenaReference,
    ctx: &LocalContext,
    module: &mut Module,
) -> DiagResult<BaseSSAValue> {
    match node.kind.clone() {
        HIRNodeKind::IntLiteral(value, size, signed) => {
            let val = build_const_int(module, value, size, signed)
                .convert(node.start.clone(), node.end.clone())?;

            Ok(val.into())
        }

        HIRNodeKind::FloatLiteral(value, size, _) => {
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

            for field in ty.get_fields() {
                vals.push(lower_hir_value(values[&field].clone(), ctx, module))
            }

            todo!()
        }

        _ => panic!(),
    }
}

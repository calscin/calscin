use calsc_ast::types::ASTType;
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_hir::HIRContext;
use calsc_typing::types::TypeKind;

pub fn lower_ast_type<S: DiagnosticSource>(
    ty: &ASTType,
    origin: &S,
    hir_ctx: &mut HIRContext,
) -> DiagResult<TypeKind> {
    calsc_tree_low::types::lower_ast_type(
        ty,
        hir_ctx.state.get_mut().get_tree_lowered_mut(),
        origin,
    )
}

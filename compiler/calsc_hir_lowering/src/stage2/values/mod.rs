//! Lowering for values

use std::hint::unreachable_unchecked;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::DiagResult;
use calsc_hir::refs::HIRArenaReference;

use crate::stage2::values::lits::lower_ast_literal;

pub mod lits;

pub fn lower_ast_value(node: ASTNode) -> DiagResult<HIRArenaReference> {
    match node.kind {
        ASTNodeKind::IntLiteral(_)
        | ASTNodeKind::FloatLiteral(_)
        | ASTNodeKind::CharLiteral(_)
        | ASTNodeKind::StringLiteral(_) => lower_ast_literal(node),

        _ => unsafe { unreachable_unchecked() },
    }
}

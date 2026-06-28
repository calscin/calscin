use std::path::PathBuf;

use calsc_ast::ASTContext;
use calsc_diagnostics::DiagPossible;
use calsc_hir::file::HIRFileContext;
use calsc_modules::tree::ModuleTree;
use calsc_typing::ctx::TypeCtx;

use crate::stage0::{append::lower_stage_0_append_pass, fill::lower_stage_0_fill_pass};

pub mod append;
pub mod fill;

pub fn lower_stage_0(
    ast_ctx: &ASTContext,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
    path: PathBuf,
) -> DiagPossible {
    let mut type_ctx = TypeCtx::new();

    lower_stage_0_append_pass(ast_ctx, file_ctx, tree, path.clone())?;
    lower_stage_0_fill_pass(ast_ctx, file_ctx, tree, path, &mut type_ctx)
}

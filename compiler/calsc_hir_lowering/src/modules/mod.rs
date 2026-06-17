use std::{fs, path::PathBuf};

use calsc_ast::parser::ctx::parse_ast_whole;
use calsc_diagnostics::{DiagPossible, DiagResult, DiagnosticSource};
use calsc_hir::{HIRContext, file::HIRFileContext};
use calsc_lexer::lexer_tokenize;
use calsc_modules::{
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};
use calsc_utils::path::to_absolute_path;

use crate::{modules::seek::seek_module_tree_folder, stage1::lower_hir_stage_1};

pub mod seek;

/// Builds the module tree based on the given building file
pub fn build_module_tree(file_path: PathBuf) -> DiagResult<ModuleTree> {
    let mut tree = ModuleTree::new();

    let file_path = to_absolute_path(file_path).unwrap();

    let folder = file_path.parent().unwrap().to_path_buf();

    let module_path = HIRFileContext::new().current_module; // Gets the current_package::empty path.

    seek_module_tree_folder(folder, module_path, &mut tree)?;

    Ok(tree)
}

pub fn module_tree_append_file(
    path: PathBuf,
    module_path: ModulePath,
    tree: &mut ModuleTree,
) -> DiagPossible {
    let lexer = lexer_tokenize(
        &fs::read_to_string(&path).unwrap(),
        path.to_str().unwrap().to_string(),
    )?;

    let ast = parse_ast_whole(&lexer)?;

    let mut hir_ctx = HIRContext::new();
    let mut hir_file_ctx = HIRFileContext::new();
    hir_file_ctx.current_module = module_path;

    Ok(())
}

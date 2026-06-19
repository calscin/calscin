use std::{fs, path::PathBuf};

use calsc_ast::parser::ctx::parse_ast_whole;
use calsc_diagnostics::{DiagPossible, DiagResult, PosDiagnosticSource};
use calsc_hir::{BUILD_CACHE, buildcache::entry::BuildCacheEntry, file::HIRFileContext};
use calsc_lexer::lexer_tokenize;
use calsc_modules::{
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};
use calsc_utils::path::to_absolute_path;

use crate::{
    modules::seek::seek_module_tree_folder,
    stage0::{fill::prelude::apply_stage0_prelude, lower_stage_0},
};

pub mod seek;

/// Builds the module tree based on the given building file
pub fn build_module_tree(file_path: PathBuf) -> DiagResult<ModuleTree> {
    let mut tree = ModuleTree::new();

    let file_path = to_absolute_path(file_path).unwrap();

    let folder = file_path.parent().unwrap().to_path_buf();

    let module_path = HIRFileContext::new().current_module; // Gets the current_package::empty path.

    // Make sure that the package path is imported and also load the tree prelude
    {
        let dummy_source = PosDiagnosticSource::new(Default::default(), Default::default());

        let mut_ref = tree.traverse_mutably_to(module_path.clone(), &dummy_source)?;

        if let ModuleTreeEntry::Module(module) = mut_ref {
            module.imported = true;
        }

        apply_stage0_prelude(&mut tree, &dummy_source)?;
    }

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

    BUILD_CACHE.with_borrow_mut(|cache| {
        cache.append_entry(path.clone(), BuildCacheEntry::new(ast.clone()))
    });

    let mut hir_file_ctx = HIRFileContext::new();
    hir_file_ctx.current_module = module_path;

    lower_stage_0(&ast, &mut hir_file_ctx, tree, path)?;

    Ok(())
}

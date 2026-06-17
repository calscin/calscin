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

    lower_hir_stage_1(ast, &mut hir_ctx, &mut hir_file_ctx)?;

    for entry in &hir_ctx.scope.key_to_ind {
        let source = &hir_ctx.scope.sources[*entry.1];

        let key = entry.0.clone();

        let visibility = &hir_ctx.scope.visibilities[*entry.1];

        if !visibility.should_be_added_to_tree() {
            continue;
        }

        let entry = &hir_ctx.scope.values[*entry.1];

        let tree_entry;

        if entry.is_function() {
            let func = entry.as_function(source)?;

            tree_entry = ModuleTreeEntry::Function(func.return_type.clone(), func.arguments.clone())
        } else if entry.is_type() {
            let ty = entry.as_type(source)?;

            tree_entry = ModuleTreeEntry::Type(ty);
        } else if entry.is_module() {
            let mut path = key.module_path.clone();
            path.path.push(key.name);

            let k = tree.traverse_mutably_to(path, source)?;

            if let ModuleTreeEntry::Module(m) = k {
                m.imported = true;
            }

            continue;
        } else {
            continue;
        }

        let mut path = key.module_path.clone();
        path.path.push(key.name);

        tree.traverse_to_append(path, tree_entry, source)?;
    }

    Ok(())
}

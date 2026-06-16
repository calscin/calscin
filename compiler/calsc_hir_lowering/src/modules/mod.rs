use std::{fs, path::PathBuf};

use calsc_ast::parser::ctx::parse_ast_whole;
use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::{HIRContext, file::HIRFileContext};
use calsc_lexer::lexer_tokenize;
use calsc_modules::{
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
    visibility::Visibility,
};

use crate::stage1::lower_hir_stage_1;

pub fn module_tree_append_file<S: DiagnosticSource>(
    path: PathBuf,
    module_path: ModulePath,
    tree: &mut ModuleTree,
    source: &S,
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
        } else {
            continue;
        }

        let mut path = key.module_path.clone();
        path.path.push(key.name);

        tree.traverse_to_append(path, tree_entry, source)?;
    }

    Ok(())
}

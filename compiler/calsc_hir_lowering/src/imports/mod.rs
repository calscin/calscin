//! The import resolving engine for the HIR lowering layer of Calscin.
//! This resolving engine mostly runs on the stage 1 layer of Calscin.

use std::{fs, path::PathBuf};

use calsc_ast::parser::ctx::parse_ast_whole;
use calsc_diagnostics::{DiagPossible, DiagResult, DiagnosticSource};
use calsc_hir::{HIRContext, file::HIRFileContext, globalctx::key::GlobalContextKey};
use calsc_lexer::lexer_tokenize;
use calsc_utils::hash::HashedString;

use crate::{
    imports::helpers::{import_function, import_type},
    stage1::lower_hir_stage_1,
};

pub mod helpers;

fn resolve_import_common(path: PathBuf) -> DiagResult<(HIRContext, HIRFileContext)> {
    let lexer = lexer_tokenize(
        &fs::read_to_string(&path).unwrap(),
        path.to_str().unwrap().to_string(),
    )?;

    let ast = parse_ast_whole(&lexer)?;

    let mut hir_ctx = HIRContext::new();
    let mut hir_file_ctx = HIRFileContext::new_with_package(
        path.with_extension("")
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .into(),
    );

    lower_hir_stage_1(ast, &mut hir_ctx, &mut hir_file_ctx)?;

    Ok((hir_ctx, hir_file_ctx))
}

pub fn resolve_import_symbols<S: DiagnosticSource>(
    path: PathBuf,
    apply_to: &mut HIRContext,
    apply_to_file: &mut HIRFileContext,
    symbols: Vec<HashedString>,
    origin: &S,
) -> DiagPossible {
    let res = resolve_import_common(path)?;

    let hir_ctx = res.0;
    let hir_file_ctx = res.1;

    for symbol in symbols {
        let key =
            GlobalContextKey::new(symbol.clone()).module_path(hir_file_ctx.current_module.clone());

        let entry = hir_ctx.scope.get_entry(key.clone(), origin)?;

        if entry.is_function() {
            let func = entry.as_function(origin)?;

            import_function(
                key,
                func.arguments.clone(),
                func.return_type.clone(),
                func.name.name.clone(),
                apply_to_file,
                origin,
                apply_to,
                apply_to_file.current_module.clone(),
            )?;
        } else {
            let ty = entry.as_type(origin)?;

            import_type(
                key,
                ty,
                apply_to_file,
                origin,
                apply_to,
                apply_to_file.current_module.clone(),
            )?;
        }
    }

    Ok(())
}

pub fn resolve_import_all<S: DiagnosticSource>(
    path: PathBuf,
    apply_to: &mut HIRContext,
    apply_to_file: &mut HIRFileContext,
    origin: &S,
) -> DiagPossible {
    let res = resolve_import_common(path)?;

    let hir_ctx = res.0;
    let hir_file_ctx = res.1;

    for entry in hir_ctx.scope.key_to_ind.clone() {
        let key = entry.0;

        if !key.module_path.path.is_empty() {
            continue; // Only import things that are inside of the package module to allow for choosing what gets exported
        }

        let value = hir_ctx.scope.get_entry(key.clone(), origin)?;

        if value.is_function() {
            let func = value.as_function(origin)?;

            import_function(
                key,
                func.arguments.clone(),
                func.return_type.clone(),
                func.name.name.clone(),
                apply_to_file,
                origin,
                apply_to,
                hir_file_ctx.current_module.clone(),
            )?;
        } else {
            let ty = value.as_type(origin)?;

            import_type(
                key,
                ty,
                apply_to_file,
                origin,
                apply_to,
                apply_to_file.current_module.clone(),
            )?;
        }
    }

    Ok(())
}

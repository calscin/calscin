//! This layer runs right after the *AST* and is mostly made to generate a module tree of the package, containing every used module in a tree structure.
//! Each module containing the following information:
//! - *Elements*: the children / symbols of the module. Can also contain child modules
//! - *Imports*: the imports used inside of the module is a table of `imported element name / path -> true full path inside of the package`
//! - *File*: the file that made this module / the file origin of the module.
//!
//! Furthermore the root of the module tree itself will store the following information:
//! - Module tree
//! - `Path -> Module Entry` resolved cache
//! - Set of files that are used for every discovered modules (`HashSet<PathBuf>`)
//!
//!
//! We also associate every entry with the related nodes, for example every type tree entry should be related to the AST node that caused it to be added / to exist.

use std::{fs, path::PathBuf};

use calsc_ast::{ASTContext, parser::ctx::parse_ast_whole};
use calsc_diagnostics::{DiagPossible, panics::PanicDiagnosticSource};
use calsc_lexer::lexer_tokenize;

use crate::{
    ctx::TreeBuildingCtx, discover::discover_files, utils::get_module_name_from_file,
    walk::walk_in_file, walk_second_pass::walk_second_pass,
};

pub mod ctx;
pub mod discover;
pub mod prelude;
pub(crate) mod utils;
pub(crate) mod walk;
pub(crate) mod walk_second_pass;

pub fn build_module_tree(path: PathBuf, ctx: &mut TreeBuildingCtx) -> DiagPossible {
    assert!(path.parent().is_some());

    let lexer = lexer_tokenize(
        &fs::read_to_string(&path).unwrap(),
        path.to_str().unwrap().to_string(),
    )?;

    let ast = parse_ast_whole(&lexer)?;

    analyze_file(path, ast, ctx)?;

    for file in &ctx.tree.used_files.clone() {
        println!("Used file: {:#?}", file);

        walk_second_pass(file, ctx)?;
    }

    Ok(())
}

pub fn analyze_file(path: PathBuf, ast: ASTContext, ctx: &mut TreeBuildingCtx) -> DiagPossible {
    ctx.current_file = path.clone();

    {
        let module_name = get_module_name_from_file(&path);

        if !module_name.is_empty() {
            ctx.current_path.append_single_bit(module_name);
        }
    }

    println!("+ Scanning {}", ctx.current_path);

    // Append the module path base to the cache
    ctx.module_path_base
        .insert(path.clone(), ctx.current_path.clone());

    // Append the module to the path
    ctx.tree.append_module(
        &ctx.current_path,
        path.clone(),
        &mut ctx.arena,
        &PanicDiagnosticSource(), // TODO: change this
    )?;

    walk_in_file(&ast, ctx)?;

    let files = discover_files(&ast, ctx, path.parent().unwrap().to_path_buf())?;

    for file in files {
        let lexer = lexer_tokenize(
            &fs::read_to_string(&file).unwrap(),
            file.to_str().unwrap().to_string(),
        )?;

        let ast = parse_ast_whole(&lexer)?;

        analyze_file(file, ast, ctx)?;
    }

    // Remove the appended module name
    ctx.current_path.path.pop();

    ctx.ast_contexts.insert(path, ast);

    Ok(())
}

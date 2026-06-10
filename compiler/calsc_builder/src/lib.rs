use std::{
    fs,
    path::PathBuf,
    process::{self, Command},
};

use calsc_ast::{AST_CONTEXT, parser::ctx::parse_ast_whole};
use calsc_diagnostics::container::dump_and_stop_if_errors;
use calsc_hir::HIR_CONTEXT;
use calsc_hir_lowering::{stage1::lower_hir_stage_1, stage2::lower_hir_stage_2};
use calsc_lexer::lexer_tokenize;
use calsc_remir_lowering::build_context_to_object_file;

pub fn build_file(path: PathBuf, out: PathBuf, pie: bool) {
    let module_name = path.file_name().unwrap().to_str().unwrap().to_string();

    let contents = match fs::read_to_string(path.clone()) {
        Ok(v) => v,
        Err(e) => {
            println!("File error! {}", e);
            process::exit(1);
        }
    };

    // Lexer

    let lexer = lexer_tokenize(&contents, path.to_str().unwrap().to_string());
    dump_and_stop_if_errors();

    let lexer = lexer.unwrap();

    // AST

    let _ = parse_ast_whole(&lexer);
    dump_and_stop_if_errors();

    let ast = AST_CONTEXT.with_borrow(|f| f.clone());

    // HIR

    let _ = lower_hir_stage_1(ast.clone());
    dump_and_stop_if_errors();

    let _ = lower_hir_stage_2(ast);
    dump_and_stop_if_errors();

    let hir = HIR_CONTEXT.with_borrow(|f| f.clone());

    // MIR

    let _ = build_context_to_object_file(hir, module_name, out, pie);
    dump_and_stop_if_errors();
}

pub fn link_files(files: Vec<PathBuf>, out: PathBuf, linker: String) {
    let mut command = Command::new(linker);

    for file in files {
        command.arg(file.to_str().unwrap());
    }

    let output = command
        .arg(format!("-o{}", out.to_str().unwrap()))
        .output()
        .unwrap();

    println!("{}", String::from_utf8_lossy(&output.stderr));
}

pub fn check_file(path: PathBuf, ast_only: bool) {
    let contents = match fs::read_to_string(path.clone()) {
        Ok(v) => v,
        Err(e) => {
            println!("File error! {}", e);
            process::exit(1);
        }
    };

    // Lexer

    let lexer = lexer_tokenize(&contents, path.to_str().unwrap().to_string());
    dump_and_stop_if_errors();

    let lexer = lexer.unwrap();

    // AST

    let _ = parse_ast_whole(&lexer);
    dump_and_stop_if_errors();

    let ast = AST_CONTEXT.with_borrow(|f| f.clone());

    // HIR

    if !ast_only {
        let _ = lower_hir_stage_1(ast.clone());
        dump_and_stop_if_errors();

        let _ = lower_hir_stage_2(ast);
        dump_and_stop_if_errors();
    }
}

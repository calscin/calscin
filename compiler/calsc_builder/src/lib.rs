use std::{fs, path::PathBuf, process};

use calsc_ast::{AST_CONTEXT, parser::ctx::parse_ast_whole};
use calsc_diagnostics::{DiagPossible, container::dump_and_stop_if_errors};
use calsc_hir::HIR_CONTEXT;
use calsc_hir_lowering::{stage1::lower_hir_stage_1, stage2::lower_hir_stage_2};
use calsc_lexer::lexer_tokenize;
use calsc_remir_lowering::build_context_to_object_file;

pub fn build_file(path: PathBuf, out: PathBuf) -> DiagPossible {
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

    let ast = AST_CONTEXT.take();

    // HIR

    let _ = lower_hir_stage_1(ast.clone());
    dump_and_stop_if_errors();

    let _ = lower_hir_stage_2(ast);
    dump_and_stop_if_errors();

    let hir = HIR_CONTEXT.take();

    // MIR

    let _ = build_context_to_object_file(hir, module_name, out);
    dump_and_stop_if_errors();

    Ok(())
}

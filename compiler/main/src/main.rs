use std::{env, fs, path::PathBuf};

use calsc_ast::{AST_CONTEXT, ASTContext, parser::ctx::parse_ast_whole};
use calsc_diagnostics::result::CalscinResult;
use calsc_hir::HIR_CONTEXT;
use calsc_hir_lowering::stage1::lower_hir_stage_1;
use calsc_lexer::lexer_tokenize;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = PathBuf::from(args[1].clone());

    let lexer = lexer_tokenize(
        &fs::read_to_string(file.clone()).unwrap(),
        file.to_str().unwrap().to_string(),
    )
    .unwrap_cleanly();

    parse_ast_whole(&lexer).unwrap_cleanly();

    let context = AST_CONTEXT.with(|f| f.clone().replace(ASTContext::new())); // Grabs the AST context

    lower_hir_stage_1(context).unwrap_cleanly();

    HIR_CONTEXT.with_borrow(|f| print!("{:#?}", f));
}

use calsc_ast::parser::vars::parse_ast_variable_declaration;
use calsc_diagnostics::result::CalscinResult;
use calsc_lexer::lexer_tokenize;

#[test]
pub fn test_parse_variable_delc_no_def() {
    let tokens = lexer_tokenize("mut s32 test", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_variable_declaration(&tokens, &mut ind).unwrap_cleanly();
}

#[test]
pub fn test_parse_variable_delc_def() {
    let tokens = lexer_tokenize("var s32 test = 45", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_variable_declaration(&tokens, &mut ind).unwrap_cleanly();
}

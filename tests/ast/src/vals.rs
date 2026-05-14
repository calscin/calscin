use calsc_ast::parser::values::parse_ast_value;
use calsc_diagnostics::result::CalscinResult;
use calsc_lexer::lexer_tokenize;

#[test]
pub fn parse_structured_init_test() {
    let tokens = lexer_tokenize("{test: 587, abc: true}", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_value(&tokens, &mut ind).unwrap_cleanly();
}

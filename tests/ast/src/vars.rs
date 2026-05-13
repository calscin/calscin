use calsc_ast::{
    nodes::ASTNodeKind,
    parser::vars::{parse_ast_variable_declaration, parse_ast_variable_reference},
};
use calsc_diagnostics::result::CalscinResult;
use calsc_lexer::lexer_tokenize;
use calsc_utils::hash::HashedString;

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

#[test]
pub fn test_parse_variable_ref() {
    let tokens = lexer_tokenize("test_abcef", "test.cal".to_string()).unwrap();
    let mut ind = 0;

    let reference = parse_ast_variable_reference(&tokens, &mut ind).unwrap_cleanly();

    assert_eq!(
        reference.kind,
        ASTNodeKind::VariableReference(HashedString::new("test_abcef".to_string()))
    )
}

use calsc_ast::{nodes::ASTNodeKind, parser::func::parse_function_declaration};
use calsc_diagnostics::result::CalscinResult;
use calsc_lexer::lexer_tokenize;
use calsc_utils::hash::HashedString;

#[test]
pub fn function_decl_parsing_base_test() {
    let tokens = lexer_tokenize("func test() {}", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let func = parse_function_declaration(&tokens, &mut ind).unwrap_cleanly();

    assert_eq!(
        func.kind,
        ASTNodeKind::FunctionDeclaration {
            name: HashedString::new("test".to_string()),
            arguments: vec![],
            body: vec![]
        }
    )
}

#[test]
pub fn function_decl_parsing_test() {
    let tokens = lexer_tokenize("func test() {\n var s32 test = 0 }", "test.cal".to_string())
        .unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_function_declaration(&tokens, &mut ind).unwrap_cleanly();
}

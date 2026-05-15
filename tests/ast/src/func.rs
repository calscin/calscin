use calsc_ast::parser::func::parse_extern_function_declaration;
#[allow(unused)]
use calsc_ast::{
    nodes::ASTNodeKind,
    parser::{
        func::{parse_function_call, parse_function_declaration},
        parse_ast_node_body_member,
    },
};
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

#[test]
pub fn function_call_parsing_test() {
    let tokens = lexer_tokenize("test()", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    assert_eq!(
        call.kind,
        ASTNodeKind::FunctionCall {
            name: HashedString::new("test".to_string()),
            arguments: vec![]
        }
    )
}

#[test]
pub fn parse_call_parsing_args_test() {
    let tokens =
        lexer_tokenize("test(testtwo(), 123, 4565)", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::FunctionCall { name, arguments } = call.kind {
        assert_eq!(name, HashedString::new("test".to_string()));

        assert_eq!(
            arguments[0].kind,
            ASTNodeKind::FunctionCall {
                name: HashedString::new("testtwo".to_string()),
                arguments: vec![]
            }
        );
        assert_eq!(arguments[1].kind, ASTNodeKind::IntLiteral(123));
        assert_eq!(arguments[2].kind, ASTNodeKind::IntLiteral(4565));
    } else {
        assert!(false)
    }
}

#[test]
pub fn parse_extern_function_decl_base_test() {
    let tokens = lexer_tokenize("externfunc test()", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_extern_function_declaration(&tokens, &mut ind).unwrap_cleanly();

    assert_eq!(
        call.kind,
        ASTNodeKind::ExternFunctionDeclaration {
            name: HashedString::new("test".to_string()),
            arguments: vec![],
            triple_dot_position: None
        }
    )
}

#[test]
pub fn parse_extern_function_decl_test() {
    let tokens = lexer_tokenize("externfunc test(...)", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_extern_function_declaration(&tokens, &mut ind).unwrap_cleanly();

    assert_eq!(
        call.kind,
        ASTNodeKind::ExternFunctionDeclaration {
            name: HashedString::new("test".to_string()),
            arguments: vec![],
            triple_dot_position: Some(0)
        }
    )
}

#[test]
pub fn parse_malformed_extern_function_decl_test() {
    let tokens =
        lexer_tokenize("externfunc test(..., s32 test)", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_extern_function_declaration(&tokens, &mut ind).unwrap_err();
}

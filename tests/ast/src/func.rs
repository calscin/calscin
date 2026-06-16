#[cfg(test)]
use calsc_ast::ASTContext;

#[cfg(test)]
use calsc_ast::parser::func::parse_extern_function_declaration;

use calsc_ast::visibility;
#[cfg(test)]
use calsc_ast::{
    nodes::ASTNodeKind,
    parser::{func::parse_function_declaration, parse_ast_node_body_member},
    types::ASTType,
};

#[cfg(test)]
use calsc_diagnostics::result::CalscinResult;

#[cfg(test)]
use calsc_ast::path::ElementPath;

#[cfg(test)]
use calsc_lexer::lexer_tokenize;

#[test]
pub fn function_decl_parsing_base_test() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("func test() {}", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let func = parse_function_declaration(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let func_ref = ctx.nodes.get(&func);

    assert_eq!(
        func_ref.kind.clone(),
        ASTNodeKind::FunctionDeclaration {
            name: "test".into(),
            arguments: vec![],
            return_type: ASTType::Void,
            body: vec![],
            visibility: None
        }
    )
}

#[test]
pub fn function_decl_parsing_test() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize(
        "func test() {\n var s32 test = 0; }",
        "test.cal".to_string(),
    )
    .unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_function_declaration(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
}

#[test]
pub fn function_call_parsing_test() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("test()", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_ast_node_body_member(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let call_ref = ctx.nodes.get(&call);

    assert_eq!(
        call_ref.kind.clone(),
        ASTNodeKind::FunctionCall {
            name: ElementPath::new_relative(vec!["test".into()]),
            arguments: vec![]
        }
    )
}

#[test]
pub fn parse_call_parsing_args_test() {
    let mut ctx = ASTContext::new();

    let tokens =
        lexer_tokenize("test(testtwo(), 123, 4565)", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_ast_node_body_member(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let call_ref = ctx.nodes.get(&call);

    if let ASTNodeKind::FunctionCall { name, arguments } = call_ref.kind.clone() {
        let arguments_0 = ctx.nodes.get(&arguments[0]);
        let arguments_1 = ctx.nodes.get(&arguments[1]);
        let arguments_2 = ctx.nodes.get(&arguments[2]);

        assert_eq!(name, ElementPath::new_relative(vec!["test".into()]));

        assert_eq!(
            arguments_0.kind.clone(),
            ASTNodeKind::FunctionCall {
                name: ElementPath::new_relative(vec!["testtwo".into()]),
                arguments: vec![]
            }
        );
        assert_eq!(arguments_1.kind.clone(), ASTNodeKind::IntLiteral(123));
        assert_eq!(arguments_2.kind.clone(), ASTNodeKind::IntLiteral(4565));
    } else {
        assert!(false)
    }
}

#[test]
pub fn parse_extern_function_decl_base_test() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("externfunc test()", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_extern_function_declaration(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let call_ref = ctx.nodes.get(&call);

    assert_eq!(
        call_ref.kind.clone(),
        ASTNodeKind::ExternFunctionDeclaration {
            name: "test".into(),
            arguments: vec![],
            return_type: ASTType::Void,
            triple_dot_position: None,
            visibility: None
        }
    )
}

#[test]
pub fn parse_extern_function_decl_test() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("externfunc test(...)", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let call = parse_extern_function_declaration(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let call_ref = ctx.nodes.get(&call);

    assert_eq!(
        call_ref.kind.clone(),
        ASTNodeKind::ExternFunctionDeclaration {
            name: "test".into(),
            arguments: vec![],
            return_type: ASTType::Void,
            triple_dot_position: Some(0),
            visibility: None
        }
    )
}

#[test]
pub fn parse_malformed_extern_function_decl_test() {
    let mut ctx = ASTContext::new();

    let tokens =
        lexer_tokenize("externfunc test(..., s32 test)", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_extern_function_declaration(&tokens, &mut ind, &mut ctx).unwrap_err();
}

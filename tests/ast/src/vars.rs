#[cfg(test)]
use calsc_ast::ASTContext;

#[cfg(test)]
use calsc_ast::parser::parse_ast_node_body_member;

#[cfg(test)]
use calsc_ast::{
    nodes::ASTNodeKind,
    parser::vars::{parse_ast_element_reference, parse_ast_variable_declaration},
};

#[cfg(test)]
use calsc_diagnostics::result::CalscinResult;

#[cfg(test)]
use calsc_lexer::lexer_tokenize;

#[test]
pub fn test_parse_variable_delc_no_def() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("mut s32 test", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_variable_declaration(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
}

#[test]
pub fn test_parse_variable_delc_def() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("var s32 test = 45", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_variable_declaration(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
}

#[test]
pub fn test_parse_variable_ref() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("test_abcef", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let reference = parse_ast_element_reference(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let reference_ref = ctx.nodes.get(&reference);

    assert_eq!(
        reference_ref.kind.clone(),
        ASTNodeKind::ElementReference("test_abcef".into())
    )
}

#[test]
pub fn test_parse_variable_assign() {
    let mut ctx = ASTContext::new();

    let tokens = lexer_tokenize("test = 588", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let assign = parse_ast_node_body_member(&tokens, &mut ind, &mut ctx)
        .unwrap()
        .unwrap_cleanly();
    let assign_ref = ctx.nodes.get(&assign);

    if let ASTNodeKind::Assignment { variable, value } = assign_ref.kind.clone() {
        let variable = ctx.nodes.get(&variable);
        let value = ctx.nodes.get(&value);

        assert_eq!(
            variable.kind.clone(),
            ASTNodeKind::ElementReference("test".into())
        );

        assert_eq!(value.kind.clone(), ASTNodeKind::IntLiteral(588));
    } else {
        panic!()
    }
}

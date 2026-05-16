use calsc_ast::{nodes::ASTNodeKind, parser::parse_ast_node_body_member, types::ASTType};
use calsc_diagnostics::result::CalscinResult;
use calsc_lexer::lexer_tokenize;
use calsc_utils::hash::HashedString;

#[test]
fn parse_for_loop_test() {
    let tokens =
        lexer_tokenize("for s32 test => [0..1]->2 {}", "test.cal".to_string()).unwrap_cleanly();

    let mut ind = 0;

    let for_loop = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::ForLoop {
        iterator_type,
        iterator_name,
        iterated,
        body,
    } = for_loop.kind
    {
        assert_eq!(iterator_type, ASTType::Generic("s32".into(), None, vec![]));
        assert_eq!(iterator_name, "test".into());
        assert_eq!(body, vec![]);

        if let ASTNodeKind::Range {
            start,
            end,
            increment,
        } = iterated.kind
        {
            assert_eq!(start.kind, ASTNodeKind::IntLiteral(0));
            assert_eq!(end.kind, ASTNodeKind::IntLiteral(1));
            assert_eq!(increment.unwrap().kind, ASTNodeKind::IntLiteral(2))
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

#[test]
fn parse_loop_test() {
    let tokens = lexer_tokenize("loop { var s32 test }", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let loop_node = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::Loop { body } = loop_node.kind {
        assert_eq!(
            body[0].kind,
            ASTNodeKind::VariableDeclaration {
                mutable: false,
                var_type: ASTType::Generic("s32".into(), None, vec![]),
                name: "test".into(),
                value: None
            }
        )
    } else {
        panic!()
    }
}

#[test]
fn parse_while_loop_test() {
    let tokens =
        lexer_tokenize("while(true) { var s32 test }", "test.cal".to_string()).unwrap_cleanly();

    let mut ind = 0;

    let while_node = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::WhileLoop { condition, body } = while_node.kind {
        assert_eq!(condition.kind, ASTNodeKind::BooleanLiteral(true));
        assert_eq!(
            body[0].kind,
            ASTNodeKind::VariableDeclaration {
                mutable: false,
                var_type: ASTType::Generic("s32".into(), None, vec![]),
                name: "test".into(),
                value: None
            }
        )
    } else {
        panic!()
    }
}

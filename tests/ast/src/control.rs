#[cfg(test)]
use calsc_ast::{
    ifs::IfStatementBranch, nodes::ASTNodeKind, parser::parse_ast_node_body_member, types::ASTType,
};

#[cfg(test)]
use calsc_diagnostics::result::CalscinResult;

#[cfg(test)]
use calsc_lexer::lexer_tokenize;

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
    } = for_loop.kind.clone()
    {
        assert_eq!(iterator_type, ASTType::Generic("s32".into(), None, vec![]));
        assert_eq!(iterator_name, "test".into());
        assert_eq!(body, vec![]);

        if let ASTNodeKind::Range {
            start,
            end,
            increment,
        } = iterated.kind.clone()
        {
            assert_eq!(start.kind.clone(), ASTNodeKind::IntLiteral(0));
            assert_eq!(end.kind.clone(), ASTNodeKind::IntLiteral(1));
            assert_eq!(increment.unwrap().kind.clone(), ASTNodeKind::IntLiteral(2))
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

#[test]
fn parse_loop_test() {
    let tokens = lexer_tokenize("loop { var s32 test; }", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let loop_node = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::Loop { body } = loop_node.kind.clone() {
        assert_eq!(
            body[0].kind.clone(),
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
        lexer_tokenize("while(true) { var s32 test; }", "test.cal".to_string()).unwrap_cleanly();

    let mut ind = 0;

    let while_node = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::WhileLoop { condition, body } = while_node.kind.clone() {
        assert_eq!(condition.kind.clone(), ASTNodeKind::BooleanLiteral(true));
        assert_eq!(
            body[0].kind.clone(),
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
fn parse_if_statement_simple_test() {
    let tokens =
        lexer_tokenize("if(true) { var s32 test; }", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let if_node = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::IfStatement { branches } = if_node.kind.clone() {
        if let IfStatementBranch::If { condition, body } = branches[0].clone() {
            assert_eq!(condition.kind.clone(), ASTNodeKind::BooleanLiteral(true));
            assert_eq!(
                body[0].kind.clone(),
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
    } else {
        panic!()
    }
}

#[test]
fn parse_if_statement_test() {
    let tokens = lexer_tokenize(
        "if(true) {} else if(false) {} else {}",
        "test.cal".to_string(),
    )
    .unwrap_cleanly();

    let mut ind = 0;

    let if_node = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::IfStatement { branches } = if_node.kind.clone() {
        if let IfStatementBranch::If { condition, body } = branches[0].clone() {
            assert_eq!(condition.kind.clone(), ASTNodeKind::BooleanLiteral(true));
            assert_eq!(body, vec![])
        } else {
            panic!()
        }

        if let IfStatementBranch::IfElse { condition, body } = branches[1].clone() {
            assert_eq!(condition.kind.clone(), ASTNodeKind::BooleanLiteral(false));
            assert_eq!(body, vec![]);
        } else {
            panic!()
        }

        if let IfStatementBranch::Else { body } = branches[2].clone() {
            assert_eq!(body, vec![]);
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

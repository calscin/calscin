use calsc_ast::{nodes::ASTNodeKind, parser::values::parse_ast_value};
use calsc_diagnostics::result::CalscinResult;
use calsc_lexer::lexer_tokenize;
use calsc_utils::{
    cmp::{CompareOperator, ComparePredicate},
    hash::HashedString,
    math::{MathOperation, MathOperator},
};

#[test]
pub fn parse_structured_init_test() {
    let tokens = lexer_tokenize("{test: 587, abc: true}", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_value(&tokens, &mut ind, true, false).unwrap_cleanly();
}

#[test]
pub fn parse_malformed_structured_init_test() {
    let tokens = lexer_tokenize("{test: 588 abc: true}", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_value(&tokens, &mut ind, true, false).unwrap_err();
}

#[test]
pub fn parse_math_operation_test() {
    let tokens = lexer_tokenize("test += 58", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let math = parse_ast_value(&tokens, &mut ind, true, false).unwrap_cleanly();

    if let ASTNodeKind::MathExpression {
        left_expr,
        right_expr,
        operator,
    } = math.kind.clone()
    {
        assert_eq!(
            left_expr.kind.clone(),
            ASTNodeKind::ElementReference(HashedString::new("test".to_string()))
        );
        assert_eq!(right_expr.kind.clone(), ASTNodeKind::IntLiteral(58));

        assert_eq!(operator, MathOperator::new(MathOperation::Add, false, true))
    } else {
        panic!()
    }
}

#[test]
pub fn parse_math_operation_long_test() {
    let tokens = lexer_tokenize("test ~++= 58", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let math = parse_ast_value(&tokens, &mut ind, true, false).unwrap_cleanly();

    if let ASTNodeKind::MathExpression {
        left_expr,
        right_expr,
        operator,
    } = math.kind.clone()
    {
        assert_eq!(
            left_expr.kind.clone(),
            ASTNodeKind::ElementReference(HashedString::new("test".to_string()))
        );
        assert_eq!(right_expr.kind.clone(), ASTNodeKind::IntLiteral(58));

        assert_eq!(operator, MathOperator::new(MathOperation::And, true, true))
    } else {
        panic!()
    }
}

#[test]
pub fn parse_compare_operation_test() {
    let tokens = lexer_tokenize("test <= 58", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let comp = parse_ast_value(&tokens, &mut ind, true, false).unwrap_cleanly();

    if let ASTNodeKind::CompareExpression {
        left_expr,
        right_expr,
        operator,
    } = comp.kind.clone()
    {
        assert_eq!(
            left_expr.kind.clone(),
            ASTNodeKind::ElementReference(HashedString::new("test".to_string()))
        );

        assert_eq!(right_expr.kind.clone(), ASTNodeKind::IntLiteral(58));

        assert_eq!(
            operator,
            CompareOperator::new(ComparePredicate::LowerThan, true)
        )
    } else {
        panic!()
    }
}

#[test]
pub fn parse_range_test() {
    let tokens = lexer_tokenize("[1..5]->5", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let val = parse_ast_value(&tokens, &mut ind, true, false).unwrap_cleanly();

    if let ASTNodeKind::Range {
        start,
        end,
        increment,
    } = val.kind.clone()
    {
        assert_eq!(start.kind.clone(), ASTNodeKind::IntLiteral(1));
        assert_eq!(end.kind.clone(), ASTNodeKind::IntLiteral(5));
        assert_eq!(increment.unwrap().kind.clone(), ASTNodeKind::IntLiteral(5));
    } else {
        panic!()
    }
}

#[test]
fn parse_lru_test() {
    let tokens = lexer_tokenize("test.abc", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let val = parse_ast_value(&tokens, &mut ind, true, false).unwrap_cleanly();

    if let ASTNodeKind::StructLRUsage {
        left_expr,
        right_expr,
    } = val.kind.clone()
    {
        assert_eq!(left_expr.kind, ASTNodeKind::ElementReference("test".into()));
        assert_eq!(right_expr.kind, ASTNodeKind::ElementReference("abc".into()));
    } else {
        panic!()
    }
}

#[test]
fn parse_lru_function_test() {
    let tokens = lexer_tokenize("test.abc()", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let val = parse_ast_value(&tokens, &mut ind, true, false).unwrap_cleanly();

    if let ASTNodeKind::StructLRUsage {
        left_expr,
        right_expr,
    } = val.kind.clone()
    {
        assert_eq!(left_expr.kind, ASTNodeKind::ElementReference("test".into()));
        assert_eq!(
            right_expr.kind,
            ASTNodeKind::FunctionCall {
                name: "abc".into(),
                arguments: vec![]
            }
        );
    } else {
        panic!()
    }
}

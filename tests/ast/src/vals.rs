use calsc_ast::path::ElementPath;
#[cfg(test)]
use calsc_ast::{
    nodes::{ASTNodeKind, BinaryOperator},
    parser::values::parse_ast_value,
};

#[cfg(test)]
use calsc_diagnostics::result::CalscinResult;

#[cfg(test)]
use calsc_lexer::lexer_tokenize;

#[cfg(test)]
use calsc_utils::{
    cmp::{CompareOperator, ComparePredicate},
    hash::HashedString,
    math::{MathOperation, MathOperator},
};

#[test]
pub fn parse_structured_init_test() {
    let tokens = lexer_tokenize("{test: 587, abc: true}", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();
}

#[test]
pub fn parse_malformed_structured_init_test() {
    let tokens = lexer_tokenize("{test: 588 abc: true}", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let _ = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_err();
}

#[test]
pub fn parse_math_operation_test() {
    let tokens = lexer_tokenize("test += 58", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let math = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();

    if let ASTNodeKind::BinaryExpression {
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

        assert_eq!(
            operator,
            BinaryOperator::Math(MathOperator::new(MathOperation::Add, false, true))
        )
    } else {
        panic!()
    }
}

#[test]
pub fn parse_math_operation_long_test() {
    let tokens = lexer_tokenize("test ~++= 58", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let math = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();

    if let ASTNodeKind::BinaryExpression {
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

        assert_eq!(
            operator,
            BinaryOperator::Math(MathOperator::new(MathOperation::And, true, true))
        )
    } else {
        panic!()
    }
}

#[test]
fn test_operator_precedence_chain() {
    // FAILS with current code:
    let tokens = lexer_tokenize("2 + 3 * 4 + 5", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let result = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();

    if let ASTNodeKind::BinaryExpression {
        left_expr: outer_left,
        right_expr: outer_right,
        operator: outer_op,
    } = result.kind.clone()
    {
        assert_eq!(
            outer_op,
            BinaryOperator::Math(MathOperator::new(MathOperation::Add, false, false)),
            "Outer operator should be +"
        );

        assert_eq!(
            outer_right.kind.clone(),
            ASTNodeKind::IntLiteral(5),
            "Outer right should be 5"
        );

        if let ASTNodeKind::BinaryExpression {
            left_expr: middle_left,
            right_expr: middle_right,
            operator: middle_op,
        } = outer_left.kind.clone()
        {
            assert_eq!(
                middle_op,
                BinaryOperator::Math(MathOperator::new(MathOperation::Add, false, false)),
                "Middle operator should be +"
            );
            assert_eq!(
                middle_left.kind.clone(),
                ASTNodeKind::IntLiteral(2),
                "Middle left should be 2"
            );

            if let ASTNodeKind::BinaryExpression {
                left_expr: inner_left,
                right_expr: inner_right,
                operator: inner_op,
            } = middle_right.kind.clone()
            {
                assert_eq!(
                    inner_op,
                    BinaryOperator::Math(MathOperator::new(MathOperation::Mul, false, false)),
                    "Inner operator should be *"
                );
                assert_eq!(inner_left.kind.clone(), ASTNodeKind::IntLiteral(3));
                assert_eq!(inner_right.kind.clone(), ASTNodeKind::IntLiteral(4));
            } else {
                panic!(
                    "FAIL: Middle right should be (3 * 4), got: {:?}",
                    middle_right.kind
                );
            }
        } else {
            panic!(
                "FAIL: Outer left should be (2 + (3 * 4)), got: {:?}",
                outer_left.kind
            );
        }
    } else {
        panic!(
            "FAIL: Result should be MathExpression, got: {:?}",
            result.kind
        );
    }
}

#[test]
pub fn parse_compare_operation_test() {
    let tokens = lexer_tokenize("test <= 58", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let comp = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();

    if let ASTNodeKind::BinaryExpression {
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
            BinaryOperator::Compare(CompareOperator::new(ComparePredicate::LowerThan, true))
        )
    } else {
        panic!()
    }
}

#[test]
pub fn parse_range_test() {
    let tokens = lexer_tokenize("[1..5]->5", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let val = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();

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

    let val = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();

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

    let val = parse_ast_value(&tokens, &mut ind, true, false, true).unwrap_cleanly();

    if let ASTNodeKind::StructLRUsage {
        left_expr,
        right_expr,
    } = val.kind.clone()
    {
        assert_eq!(left_expr.kind, ASTNodeKind::ElementReference("test".into()));
        assert_eq!(
            right_expr.kind,
            ASTNodeKind::FunctionCall {
                name: ElementPath::new_relative(vec!["abc".into()]),
                arguments: vec![]
            }
        );
    } else {
        panic!()
    }
}

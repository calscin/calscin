use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{
    alloc::arena::ArenaAllocatorReference,
    cmp::{CompareOperator, ComparePredicate},
    pos::FilePosition,
};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::values::parse_ast_value,
};

pub fn parse_ast_inverse_condition(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ArenaAllocatorReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // !

    let val = parse_ast_value(tokens, ind, false, false)?; // Does not parse post. The parse_ast_value statement of parse_inverse_condition has priority
    // Auto increments

    let end = tokens[*ind - 1].end.clone(); // Counters the auto increment to get the end

    let node = ASTNode::new(ASTNodeKind::InverseCondition(val), start, end);

    Ok(node.push())
}

#[inline(always)]
pub fn parse_ast_comparing_operator(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<CompareOperator> {
    let operator_kind = match tokens[*ind].kind {
        TokenKind::Equal => {
            *ind += 1; // first =
            tokens[*ind].expects(TokenKind::Equal)?;

            ComparePredicate::Equal
        }

        TokenKind::Bang => {
            *ind += 1; // !
            tokens[*ind].expects(TokenKind::Equal)?;

            ComparePredicate::NotEqual
        }

        TokenKind::AngelBracketOpen => ComparePredicate::LowerThan,
        TokenKind::AngelBracketClose => ComparePredicate::GreaterThan,

        _ => return Err(build_unexpected_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    };

    *ind += 1; // post kind increment

    let mut also_equals = false;

    if operator_kind != ComparePredicate::Equal && operator_kind != ComparePredicate::NotEqual {
        if tokens[*ind].kind == TokenKind::Equal {
            *ind += 1; // =
            also_equals = true;
        }
    }

    let res = match operator_kind {
        ComparePredicate::Equal => CompareOperator::new_equal(),
        ComparePredicate::NotEqual => CompareOperator::new_not_equal(),
        _ => CompareOperator::new(operator_kind, also_equals),
    };

    Ok(res)
}

/// Parses a comparing expression
#[inline(always)]
pub fn parse_ast_compare_expression(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first: ArenaAllocatorReference,
    start: FilePosition,
) -> DiagResult<ArenaAllocatorReference> {
    let operator = parse_ast_comparing_operator(tokens, ind)?; // Auto increments

    let value = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(
        ASTNodeKind::CompareExpression {
            left_expr: first,
            right_expr: value,
            operator,
        },
        start,
        end,
    );

    Ok(node.push())
}

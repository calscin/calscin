use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{
    math::{MathOperation, MathOperator},
    pos::FilePosition,
};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::values::parse_ast_value,
    refs::ASTArenaReference,
};

#[inline(always)]
pub fn parse_ast_math_operator(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<MathOperator> {
    let mut fast = false;
    let mut assigns = false;

    if tokens[*ind].kind == TokenKind::Tilde {
        *ind += 1;
        fast = true;
    }

    let operator = match &tokens[*ind].kind {
        TokenKind::Plus => {
            if tokens[*ind + 1].kind == TokenKind::Plus {
                *ind += 1; // first +

                MathOperation::And
            } else {
                MathOperation::Add
            }
        }

        TokenKind::Minus => {
            if tokens[*ind + 1].kind == TokenKind::Minus {
                *ind += 1; // first -

                MathOperation::Or
            } else {
                MathOperation::Add
            }
        }

        TokenKind::Star => MathOperation::Mul,
        TokenKind::Slash => MathOperation::Div,
        TokenKind::BackSlash => MathOperation::Mod,

        TokenKind::Bang => {
            *ind += 1; // first !

            tokens[*ind].expects(TokenKind::Bang)?;
            MathOperation::Xor
        }

        _ => return Err(build_unexpected_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    };

    *ind += 1; // post increment for every second character / single character operation

    if tokens[*ind].kind == TokenKind::Equal {
        assigns = true;
        *ind += 1; // =
    }

    Ok(MathOperator::new(operator, fast, assigns))
}

/// Parses a math operation / expression
#[inline(always)]
pub fn parse_ast_math_expression(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first_node: ASTArenaReference,
    start: FilePosition,
) -> DiagResult<ASTArenaReference> {
    let operator = parse_ast_math_operator(tokens, ind)?; // Auto increments

    let second = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(
        ASTNodeKind::MathExpression {
            left_expr: first_node,
            right_expr: second,
            operator,
        },
        start,
        end,
    );

    Ok(node.push())
}

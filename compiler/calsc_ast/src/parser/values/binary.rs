//! Parsing for binary operators and operations

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::math::MathOperation;

use crate::{
    nodes::BinaryOperator,
    parser::values::{conditions::parse_ast_comparing_operator, math::parse_ast_math_operator},
};

/// Represents the precedence or weight of an operator. The bigger it is the more it will be picked up before another one.
pub enum Precedence {
    Assignment = 1,
    LogicalOr = 2,
    LogicalAnd = 3,
    BitwiseOr = 4,
    BitwiseAnd = 6,
    Comparing = 7,
    BitwiseShift = 8,
    Addition = 9,        // +, -
    Multiplication = 10, // *, /, %
}

impl Precedence {
    pub fn get_from_operator(operator: BinaryOperator) -> Precedence {
        match operator {
            BinaryOperator::Compare(op) => Precedence::Comparing,
            BinaryOperator::Math(op) => match op.operation {
                MathOperation::Add | MathOperation::Sub => Precedence::Addition,
                MathOperation::And => Precedence::BitwiseAnd,
                MathOperation::Or | MathOperation::Nor | MathOperation::Xor => {
                    Precedence::BitwiseOr
                }
                MathOperation::ShiftLeft | MathOperation::ShiftRight => Precedence::BitwiseShift,
                MathOperation::Mul | MathOperation::Div | MathOperation::Mod => {
                    Precedence::Multiplication
                }
            },
        }
    }
}

pub fn parse_binary_comparing_operator(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<BinaryOperator> {
    match &tokens[*ind].kind {
        TokenKind::Equal | TokenKind::AngelBracketOpen | TokenKind::AngelBracketClose => Ok(
            BinaryOperator::Compare(parse_ast_comparing_operator(tokens, ind)?),
        ),

        TokenKind::Bang => {
            if tokens[*ind + 1].kind == TokenKind::Bang {
                Ok(BinaryOperator::Math(parse_ast_math_operator(tokens, ind)?))
            } else {
                Ok(BinaryOperator::Compare(parse_ast_comparing_operator(
                    tokens, ind,
                )?))
            }
        }

        TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::BackSlash => Ok(BinaryOperator::Math(parse_ast_math_operator(tokens, ind)?)),

        tok => return Err(build_unexpected_error(tok, &tokens[*ind]).into()),
    }
}

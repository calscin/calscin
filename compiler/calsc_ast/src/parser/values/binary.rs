//! Parsing for binary operators and operations

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_token_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{math::MathOperation, pos::FilePosition};

use crate::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind, BinaryOperator},
    parser::{
        utils::peek_ahead,
        values::{
            conditions::parse_ast_comparing_operator, math::parse_ast_math_operator,
            parse_ast_value,
        },
    },
    refs::ASTArenaReference,
};

/// Represents the precedence or weight of an operator. The bigger it is the more it will be picked up before another one.
#[derive(Clone)]
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
            BinaryOperator::Compare(_) => Precedence::Comparing,
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

pub fn parse_binary_operator(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<BinaryOperator> {
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

        TokenKind::Tilde
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::BackSlash => Ok(BinaryOperator::Math(parse_ast_math_operator(tokens, ind)?)),

        tok => return Err(build_unexpected_token_error(tok, &tokens[*ind]).into()),
    }
}

pub fn is_binary_operator(tokens: &Vec<Token>, ind: usize) -> bool {
    peek_ahead(tokens, ind, parse_binary_operator).0.is_ok() // TODO: maybe make this better
}

pub fn parse_ast_binary_operation(
    tokens: &Vec<Token>,
    ind: &mut usize,
    mut left: ASTArenaReference,
    start: FilePosition,
    min_precedence: Precedence,
    ctx: &mut ASTContext,
) -> DiagResult<ASTArenaReference> {
    let min_precedence = min_precedence as usize;

    loop {
        if !is_binary_operator(tokens, *ind) {
            break;
        }

        let binary_operator = peek_ahead(tokens, *ind, parse_binary_operator).0?;
        let precedence = Precedence::get_from_operator(binary_operator) as usize;

        if precedence < min_precedence {
            // We break if the precedence is lower than the minimum precedence
            break;
        }

        let operator_start = tokens[*ind].start.clone();

        let binary_operator = parse_binary_operator(tokens, ind)?;

        let mut right = parse_ast_value(tokens, ind, true, false, false, ctx)?;

        if is_binary_operator(tokens, *ind) {
            if let Ok(next_operator) = peek_ahead(tokens, *ind, parse_binary_operator).0 {
                let next_precedence = Precedence::get_from_operator(next_operator);

                if next_precedence.clone() as usize > precedence {
                    right = parse_ast_binary_operation(
                        tokens,
                        ind,
                        right,
                        operator_start.clone(),
                        next_precedence,
                        ctx,
                    )?;
                }
            }
        }

        let end = tokens[*ind - 1].end.clone();

        left = ASTNode::new(
            ASTNodeKind::BinaryExpression {
                left_expr: left,
                right_expr: right,
                operator: binary_operator,
            },
            start.clone(),
            end,
        )
        .push(ctx);
    }

    Ok(left)
}

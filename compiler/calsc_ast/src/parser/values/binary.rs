//! Parsing for binary operators and operations

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    nodes::BinaryOperator,
    parser::values::{conditions::parse_ast_comparing_operator, math::parse_ast_math_operator},
};

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

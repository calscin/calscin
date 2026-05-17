//! Common forms with utility methods to parse them

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    nodes::ASTNode,
    parser::{parse_ast_body, types::parse_ast_type, values::parse_ast_value},
    types::ASTType,
};

/// Parses where a condition should be put
/// Should only be used for conditions with the parens.
pub fn parse_ast_condition_form(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    tokens[*ind].expects(TokenKind::ParenOpen)?;
    *ind += 1; // (

    let value = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    tokens[*ind].expects(TokenKind::ParenClose)?;
    *ind += 1; // )

    Ok(value)
}

/// Parses where a body should be put
/// Should only be used for bodies with the braces
pub fn parse_ast_body_form(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Vec<Box<ASTNode>>> {
    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    let body = parse_ast_body(tokens, ind)?; // Auto increments

    Ok(body)
}

/// Parses a field-like (a type then a variable name).
pub fn parse_ast_field_form(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<(ASTType, HashedString)> {
    let ty = parse_ast_type(tokens, ind, true)?; // Auto increments

    let name = tokens[*ind].expects_keyword()?;
    *ind += 1; // keyword (variable name)

    Ok((ty, name.into()))
}

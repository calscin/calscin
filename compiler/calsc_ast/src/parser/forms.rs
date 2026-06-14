//! Common forms with utility methods to parse them

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    parser::{parse_ast_body, types::parse_ast_type, values::parse_ast_value},
    path::ElementPath,
    refs::ASTArenaReference,
    types::ASTType,
};

/// Parses where a condition should be put
/// Should only be used for conditions with the parens.
pub fn parse_ast_condition_form(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ASTArenaReference> {
    tokens[*ind].expects(TokenKind::ParenOpen)?;
    *ind += 1; // (

    let value = parse_ast_value(tokens, ind, true, false, true)?; // Auto increments

    tokens[*ind].expects(TokenKind::ParenClose)?;
    *ind += 1; // )

    Ok(value)
}

/// Parses where a body should be put
/// Should only be used for bodies with the braces
pub fn parse_ast_body_form(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Vec<ASTArenaReference>> {
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

/// Parses a return type form
pub fn parse_ast_return_type_form(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ASTType> {
    if tokens[*ind].kind == TokenKind::Minus {
        *ind += 1;
        tokens[*ind].expects(TokenKind::AngelBracketClose)?;
        *ind += 1;

        return Ok(parse_ast_type(tokens, ind, true)?);
    }

    Ok(ASTType::Void)
}

pub fn parse_element_path_form(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ElementPath> {
    let mut path: Vec<HashedString> = vec![];

    path.push(tokens[*ind].expects_keyword()?.into());
    *ind += 1; // first element

    while tokens[*ind].kind == TokenKind::Colon {
        *ind += 1; // first :

        tokens[*ind].expects(TokenKind::Colon)?;
        *ind += 1; // second :

        let val = tokens[*ind].expects_keyword()?;
        path.push(val.into());

        *ind += 1; // keyword
    }

    Ok(ElementPath { members: path })
}

//! Function related parsing

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{parse_body, types::parse_type, utils::parse_ast_list},
    types::ASTType,
};

#[inline(always)]
pub fn parse_function_argument(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<(ASTType, HashedString)> {
    let arg_type = parse_type(tokens, ind)?; // Auto increments

    let name = tokens[*ind].expects_keyword()?;

    // No post increment since this is a parse_ast_list function

    Ok((arg_type, HashedString::new(name)))
}

#[inline(always)]
pub fn parse_function_declaration(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // func

    let func_name = tokens[*ind].expects_keyword()?;
    *ind += 1; // keyword

    tokens[*ind].expects(TokenKind::ParenOpen)?;
    *ind += 1; // (

    let arguments = parse_ast_list(
        tokens,
        ind,
        parse_function_argument,
        TokenKind::ParenClose,
        false,
    )?; // Auto increments

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    let body = parse_body(tokens, ind)?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::FunctionDeclaration {
            name: HashedString::new(func_name),
            arguments,
            body,
        },
        start,
        end,
    )))
}

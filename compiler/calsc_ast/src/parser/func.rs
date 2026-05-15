//! Function related parsing

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{Either, hash::HashedString};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{
        parse_ast_body, types::parse_ast_type, utils::parse_ast_list, values::parse_ast_value,
    },
    types::ASTType,
};

/// Parses a function declaration argument
#[inline(always)]
pub fn parse_function_argument(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<(ASTType, HashedString)> {
    let arg_type = parse_ast_type(tokens, ind)?; // Auto increments

    let name = tokens[*ind].expects_keyword()?;

    // No post increment since this is a parse_ast_list function

    Ok((arg_type, HashedString::new(name)))
}

/// Parses a function declaration
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
        &mut parse_function_argument,
        TokenKind::ParenClose,
        false,
        true,
    )?; // Auto increments

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    let body = parse_ast_body(tokens, ind)?; // Auto increments

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

pub fn parse_function_call(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    let name = HashedString::new(tokens[*ind].expects_keyword()?);
    *ind += 1; // keyword

    tokens[*ind].expects(TokenKind::ParenOpen)?;
    *ind += 1; // (

    let arguments = parse_ast_list(
        tokens,
        ind,
        &mut |toks, ind| parse_ast_value(toks, ind, true, false),
        TokenKind::ParenClose,
        false,
        false, // Doesn't post increment inside of the `parse_ast_list` function since `parse_ast_value` already does it
    )?;

    let end = tokens[*ind - 1].end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::FunctionCall { name, arguments },
        start,
        end,
    )))
}

/// Parses an extern function arguments.
/// Returns an [`Either`] of value [`A`] if the argument is a triple dot argument.
/// Returns an [`Either`] of value [`B`] if the argument is an actual argument
pub fn parse_extern_function_argument(
    tokens: &Vec<Token>,
    ind: &mut usize,
    already_consumed_triple_dot: &mut bool,
) -> DiagResult<Either<(), (ASTType, HashedString)>> {
    if *already_consumed_triple_dot {
        return Err(build_unexpected_error(&tokens[*ind].kind, &tokens[*ind]).into());
    }

    if tokens[*ind].kind == TokenKind::Dot {
        *ind += 1; // first .

        tokens[*ind].expects(TokenKind::Dot)?;
        *ind += 1; // second dot

        tokens[*ind].expects(TokenKind::Dot)?; // We don't post increment here for the parse_ast_list function

        *already_consumed_triple_dot = true;

        return Ok(Either::A(()));
    } else {
        return Ok(Either::B(parse_function_argument(tokens, ind)?));
    }
}

pub fn parse_extern_function_declaration(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // externfunc

    let name = HashedString::new(tokens[*ind].expects_keyword()?);
    *ind += 1; // keyword

    tokens[*ind].expects(TokenKind::ParenOpen)?;
    *ind += 1; // (

    let mut already_consumed_triple_dot = false;
    let arguments = parse_ast_list(
        tokens,
        ind,
        &mut |toks, ind| {
            parse_extern_function_argument(toks, ind, &mut already_consumed_triple_dot)
        },
        TokenKind::ParenClose,
        false,
        true,
    )?;

    let end = tokens[*ind - 1].end.clone(); // Removes the increment to get end since parse_ast_list post increments after list parsing

    let mut args: Vec<(ASTType, HashedString)> = vec![];
    let mut triple_dot_pos: Option<usize> = None;

    // Extracts the triple dot position since it is guaranteed to be only one

    let mut arg_ind = 0;
    for argument in &arguments {
        if argument.is_a() {
            triple_dot_pos = Some(arg_ind);
        } else {
            args.push(argument.clone().unwrap_b())
        }

        arg_ind += 1;
    }

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::ExternFunctionDeclaration {
            name,
            arguments: args,
            triple_dot_position: triple_dot_pos,
        },
        start,
        end,
    )))
}

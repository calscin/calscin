//! Parsing related to types

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_token_error};
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    parser::{forms::parse_element_path_form, utils::parse_ast_list},
    types::{ASTType, SimpleASTType},
};

/// Parses the tokens as an AST type ([`ASType`][`crate::types::ASTType`])
///
/// Uses a two stage algorithm to parse the type:
/// 1. The simple type stage: A stage where types just hold information and are stored in a vector rather than a tree like storage
/// 2. The actual type stage: The stage where simple types that are stored in an array gets converted into actual [`ASType`][`crate::types::ASTType`]) objects
///
/// # Errors
/// This function will error if the parsing of the type is invalid at any step of the parsing (regardless of the stage).
///
/// # Example
/// ```
/// use calsc_ast::parser::types::parse_ast_type;
/// use calsc_lexer::lexer_tokenize;
/// use calsc_lexer::toks::{Token, TokenKind};
///
/// let tokens: Vec<Token> = lexer_tokenize("s32**", "test.cal".to_string()).unwrap();
/// let mut ind = 0;
///
/// let ty = parse_ast_type(&tokens, &mut ind, true).unwrap();
/// ```
///
pub fn parse_ast_type(
    tokens: &Vec<Token>,
    ind: &mut usize,
    allow_generic_parameters: bool,
) -> DiagResult<ASTType> {
    let mut simples = vec![];
    let mut already_parsed_generic = false;

    loop {
        let parsed_type = parse_type_step(
            tokens,
            ind,
            &mut already_parsed_generic,
            allow_generic_parameters,
        )?;

        if let Some(parsed_type) = parsed_type {
            simples.push(parsed_type);
        } else {
            break;
        }
    }

    let len = simples.len() - 1;

    Ok(lower_simple_type(simples, len))
}

/// The function used in [`parse_type`][`crate::parser::types::parse_type`]'s second stage
/// in order to lower simple types to actual AST types.
///
/// # Errors
/// **This function will never output any errors or panics**
///
#[inline(always)]
pub(crate) fn lower_simple_type(simples: Vec<SimpleASTType>, ind: usize) -> ASTType {
    // We do not need to check the index since it cannot go deper than 0. Furthermore, it is guaranteed that a generic will be at the first
    let res = match &simples[ind] {
        SimpleASTType::Generic(name, size_specifier, type_params) => {
            ASTType::Generic(name.clone(), size_specifier.clone(), type_params.clone())
        }

        SimpleASTType::Array(size) => {
            ASTType::Array(*size, Box::new(lower_simple_type(simples, ind - 1)))
        }

        SimpleASTType::Reference(mutable) => {
            ASTType::Reference(*mutable, Box::new(lower_simple_type(simples, ind - 1)))
        }
    };

    res
}

/// The function that actually does the type parsing in [`parse_type`][`crate::parser::types::parse_type`]'s first stage.
/// Parses the actual lexer tokens into [`SimpleASTType`]
///
///
/// # Returns
/// Returns a [`None`] whenever the parsing for the type ended. Meaning that the type cannot possibly be parsed this way
///
/// Returns a [`Some`] whenever the parsing for the type was a success and that a type could be parsed this way.
///
/// # Errors
///
/// This function may error out if the type parsing fails when trying to parse an array or generic type.
///
///
#[inline(always)]
pub(crate) fn parse_type_step(
    tokens: &Vec<Token>,
    ind: &mut usize,
    already_parsed_generic: &mut bool,
    allow_generic_parameters: bool,
) -> DiagResult<Option<SimpleASTType>> {
    let kind = match &tokens[*ind].kind {
        TokenKind::And => {
            *ind += 1; // &

            SimpleASTType::Reference(true)
        }
        TokenKind::BracketOpen => {
            *ind += 1; // [

            let mut size = None;

            if tokens[*ind].is_int_lit() {
                size = Some(tokens[*ind].expects_int_lit()? as usize);
                *ind += 1; // int literal
            }

            tokens[*ind].expects(TokenKind::BracketClose)?;
            *ind += 1; // ]

            SimpleASTType::Array(size)
        }

        TokenKind::Keyword(_) => {
            if *already_parsed_generic {
                return Ok(None);
            }

            *already_parsed_generic = true;
            return Ok(Some(parse_type_generic(
                tokens,
                ind,
                allow_generic_parameters,
            )?));
        }

        _ => {
            return Ok(None);
        }
    };

    Ok(Some(kind))
}

/// Function used by [`parse_type_step`][`crate::parser::types::parse_type_step`] in order to parse type generics as [`SimpleASTType`] values.
///
/// Is part of the functions used by [`parse_ast_type`][`crate::parser::types::parse_ast_type`] in order to parse types
///
/// # Errors
/// This function will error out if the parsing goes wrong.
///
#[inline(always)]
pub fn parse_type_generic(
    tokens: &Vec<Token>,
    ind: &mut usize,
    allow_generic_parameters: bool,
) -> DiagResult<SimpleASTType> {
    if let TokenKind::Keyword(_) = tokens[*ind].kind.clone() {
        let name = parse_element_path_form(tokens, ind)?;

        let size_spec;
        let mut type_parameters: Vec<ASTType> = vec![];

        // Parsing of the size specifier
        if tokens[*ind].kind == TokenKind::Dot {
            if !allow_generic_parameters {
                return Err(build_unexpected_token_error(&tokens[*ind].kind, &tokens[*ind]).into());
            }

            *ind += 1; // .

            size_spec = Some(tokens[*ind].expects_int_lit()? as usize);
            *ind += 1; // int literal
        } else {
            size_spec = None;
        }

        if tokens[*ind].kind == TokenKind::AngelBracketOpen {
            if !allow_generic_parameters {
                return Err(build_unexpected_token_error(&tokens[*ind].kind, &tokens[*ind]).into());
            }

            *ind += 1; // <

            type_parameters = parse_ast_list(
                tokens,
                ind,
                &mut |toks, i| parse_ast_type(toks, i, true),
                TokenKind::AngelBracketClose,
                true,
                false,
            )?; // Auto increments
        }

        return Ok(SimpleASTType::Generic(name, size_spec, type_parameters));
    }

    panic!("Invalid node")
}

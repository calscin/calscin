//! Parsing related to types

use calsc_diagnostics::{
    DiagResult,
    diags::errors::{build_expected_error, build_unexpected_error},
};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    parser::utils::parse_ast_list,
    types::{ASTType, SimpleASTType},
};

pub fn parse_type(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ASTType> {
    let mut simples = vec![];

    loop {
        let parsed_type = parse_type_step(tokens, ind)?;

        if let Some(parsed_type) = parsed_type {
            simples.push(parsed_type);
        } else {
            break;
        }
    }

    let len = simples.len();

    lower_simple_type(simples, len)
}

pub fn lower_simple_type(simples: Vec<SimpleASTType>, ind: usize) -> DiagResult<ASTType> {
    // We do not need to check the index since it cannot go deper than 0. Furthermore, it is guaranteed that a generic will be at the first
    let res = match &simples[ind] {
        SimpleASTType::Generic(name, size_specifier, type_params) => {
            ASTType::Generic(name.clone(), size_specifier.clone(), type_params.clone())
        }

        SimpleASTType::Array(size) => {
            ASTType::Array(*size, Box::new(lower_simple_type(simples, ind - 1)?))
        }

        SimpleASTType::Pointer => ASTType::Pointer(Box::new(lower_simple_type(simples, ind - 1)?)),
    };

    Ok(res)
}

pub fn parse_type_step(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Option<SimpleASTType>> {
    let kind = match &tokens[*ind].kind {
        TokenKind::Star => {
            *ind += 1; // *

            SimpleASTType::Pointer
        }
        TokenKind::BracketOpen => {
            *ind += 1; // [

            let size = tokens[*ind].expects_int_lit()?;
            *ind += 1; // int literal

            tokens[*ind].expects(TokenKind::BracketClose)?;
            *ind += 1; // ]

            SimpleASTType::Array(size as usize)
        }

        TokenKind::Keyword(_) => return Ok(Some(parse_type_generic(tokens, ind)?)),

        _ => return Ok(None),
    };

    Ok(Some(kind))
}

pub fn parse_type_generic(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<SimpleASTType> {
    if let TokenKind::Keyword(name) = tokens[*ind].kind.clone() {
        *ind += 1; // keyword

        let size_spec;
        let mut type_parameters: Vec<String> = vec![];

        // Parsing of the size specifier
        if tokens[*ind].kind == TokenKind::Dot {
            *ind += 1; // .

            size_spec = Some(tokens[*ind].expects_int_lit()? as usize);
            *ind += 1; // int literal
        } else {
            size_spec = None;
        }

        if tokens[*ind].kind == TokenKind::AngelBracketOpen {
            *ind += 1; // <

            type_parameters = parse_ast_list(
                tokens,
                ind,
                |toks, i| toks[*i].expects_string_lit(),
                TokenKind::AngelBracketClose,
                true,
            )?;
        }

        return Ok(SimpleASTType::Generic(
            HashedString::new(name),
            size_spec,
            type_parameters,
        ));
    }

    panic!("Invalid node")
}

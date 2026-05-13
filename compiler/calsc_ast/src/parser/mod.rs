//! The parser of the Calscin AST.
//!
//! # Guidelines
//! Individual parsing functions should always post-increment unless specified otherwise.

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};

use crate::{nodes::ASTNode, parser::vars::parse_ast_variable_declaration};

pub mod types;
pub mod utils;
pub mod values;
pub mod vars;

/// Parses a member of a function block. A function block is most of the time refereing to a function body.
///
///
///
pub fn parse_node_body_member(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    match tokens[*ind].kind {
        TokenKind::Var | TokenKind::Mut => parse_ast_variable_declaration(tokens, ind),

        _ => return Err(build_unexpected_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    }
}

pub fn parse_body(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Vec<Box<ASTNode>>> {
    let mut members: Vec<Box<ASTNode>> = vec![];

    *ind += 1; // {

    while tokens[*ind].kind != TokenKind::BraceClose {
        let member = parse_node_body_member(tokens, ind)?;

        members.push(member);
    }

    *ind += 1; // }

    Ok(members)
}

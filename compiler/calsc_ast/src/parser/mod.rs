//! The parser of the Calscin AST.
//!
//! # Guidelines
//! Individual parsing functions should always post-increment unless specified otherwise.

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    nodes::ASTNode,
    parser::{
        control::{
            for_loop::parse_ast_for_loop, ifelse::parse_ast_if_statement, loops::parse_ast_loop,
            while_loop::parse_ast_while_loop,
        },
        func::{parse_extern_function_declaration, parse_function_declaration},
        values::parse_ast_value,
        vars::parse_ast_variable_declaration,
    },
};

pub mod control;
pub mod forms;
pub mod func;
pub mod import;
pub mod types;
pub mod utils;
pub mod values;
pub mod vars;

/// Parses a member of a function block. A function block is most of the time refereing to a function body.
///
/// # Errors
/// This function will error if the starting token cannot possibly be from a body node.
///
/// This function will error if the sub parsing function fails.
///
/// # Example
/// ```
/// use calsc_ast::parser::parse_ast_node_body_member;
/// use calsc_lexer::lexer_tokenize;
/// use calsc_lexer::toks::{Token, TokenKind};
/// use calsc_diagnostics::result::CalscinResult;
///
/// let tokens: Vec<Token> = lexer_tokenize("var s32 test = 5", "test.cal".to_string()).unwrap();
/// let mut ind: usize = 0;
///
/// let node = parse_ast_node_body_member(&tokens, &mut ind).unwrap_cleanly();
/// ```
///
pub fn parse_ast_node_body_member(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    match tokens[*ind].kind {
        TokenKind::Var | TokenKind::Mut => parse_ast_variable_declaration(tokens, ind),
        TokenKind::For => parse_ast_for_loop(tokens, ind),
        TokenKind::Loop => parse_ast_loop(tokens, ind),
        TokenKind::While => parse_ast_while_loop(tokens, ind),
        TokenKind::If => parse_ast_if_statement(tokens, ind),
        _ => parse_ast_value(tokens, ind, true, true),
    }
}

pub fn parse_ast_body(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Vec<Box<ASTNode>>> {
    let mut members: Vec<Box<ASTNode>> = vec![];

    while tokens[*ind].kind != TokenKind::BraceClose {
        let member = parse_ast_node_body_member(tokens, ind)?; // Auto increments

        if !member.kind.is_body() {
            tokens[*ind].expects(TokenKind::SemiColon)?;
            *ind += 1; // ;
        }

        members.push(member);
    }

    *ind += 1; // }

    Ok(members)
}

/// Parses a top level node
pub fn parse_ast_top_level(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    match tokens[*ind].kind {
        TokenKind::Function => parse_function_declaration(tokens, ind),
        TokenKind::ExternFunc => parse_extern_function_declaration(tokens, ind),

        _ => return Err(build_unexpected_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    }
}

//! The parser of the Calscin AST.
//!
//! # Guidelines
//! Individual parsing functions should always post-increment unless specified otherwise.

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    nodes::ASTNode,
    parser::{
        control::{
            for_loop::parse_ast_for_loop, loops::parse_ast_loop, while_loop::parse_ast_while_loop,
        },
        values::parse_ast_value,
        vars::parse_ast_variable_declaration,
    },
};

pub mod control;
pub mod func;
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
        _ => parse_ast_value(tokens, ind, true, true),
    }
}

pub fn parse_ast_body(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Vec<Box<ASTNode>>> {
    let mut members: Vec<Box<ASTNode>> = vec![];

    while tokens[*ind].kind != TokenKind::BraceClose {
        let member = parse_ast_node_body_member(tokens, ind)?;

        members.push(member);
    }

    *ind += 1; // }

    Ok(members)
}

//! The parser of the Calscin AST.
//!
//! # Guidelines
//! Individual parsing functions should always post-increment unless specified otherwise.

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_token_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{
        control::{
            for_loop::parse_ast_for_loop, ifelse::parse_ast_if_statement, loops::parse_ast_loop,
            while_loop::parse_ast_while_loop,
        },
        func::{parse_extern_function_declaration, parse_function_declaration},
        import::parse_ast_import_statement,
        structs::{parse_ast_struct_decl_block, parse_ast_struct_declaration},
        values::parse_ast_value,
        vars::parse_ast_variable_declaration,
    },
    refs::ASTArenaReference,
};

pub mod control;
pub mod ctx;
pub mod forms;
pub mod func;
pub mod import;
pub mod lru;
pub mod structs;
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
) -> DiagResult<ASTArenaReference> {
    match tokens[*ind].kind {
        TokenKind::Var | TokenKind::Mut => parse_ast_variable_declaration(tokens, ind),
        TokenKind::For => parse_ast_for_loop(tokens, ind),
        TokenKind::Loop => parse_ast_loop(tokens, ind),
        TokenKind::While => parse_ast_while_loop(tokens, ind),
        TokenKind::If => parse_ast_if_statement(tokens, ind),
        TokenKind::Return => parse_ast_return_statement(tokens, ind),
        _ => parse_ast_value(tokens, ind, true, true, true),
    }
}

pub fn parse_ast_return_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // return

    let val;

    if tokens[*ind].kind == TokenKind::SemiColon {
        val = None;
        *ind += 1; // ;
    } else {
        val = Some(parse_ast_value(tokens, ind, true, false, true)?);
    }

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(ASTNodeKind::ReturnStatement { val }, start, end);

    Ok(node.push())
}

/// Parses an AST body
pub fn parse_ast_body(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Vec<ASTArenaReference>> {
    let mut members: Vec<ASTArenaReference> = vec![];

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
pub fn parse_ast_top_level(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ASTArenaReference> {
    match tokens[*ind].kind {
        TokenKind::Function => parse_function_declaration(tokens, ind),
        TokenKind::ExternFunc => parse_extern_function_declaration(tokens, ind),
        TokenKind::Struct => parse_ast_struct_declaration(tokens, ind),
        TokenKind::Decl => parse_ast_struct_decl_block(tokens, ind),
        TokenKind::Import => parse_ast_import_statement(tokens, ind),
        TokenKind::Module => parse_ast_module(tokens, ind),

        _ => return Err(build_unexpected_token_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    }
}

pub fn parse_ast_module(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // module

    let name: HashedString = tokens[*ind].expects_keyword()?.into(); // Auto increments
    let mut body = vec![];

    *ind += 1; // name

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    while tokens[*ind].kind != TokenKind::BraceClose {
        body.push(parse_ast_top_level(tokens, ind)?);
    }

    let end = tokens[*ind].end.clone();

    *ind += 1; // }

    let node = ASTNode::new(ASTNodeKind::Module { name, body }, start, end);

    Ok(node.push())
}

//! The parser of the Calscin AST.
//!
//! # Guidelines
//! Individual parsing functions should always post-increment unless specified otherwise.

use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_token_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
    parser::{
        control::{
            for_loop::parse_ast_for_loop, ifelse::parse_ast_if_statement, loops::parse_ast_loop,
            while_loop::parse_ast_while_loop,
        },
        ctx::CommentContext,
        enums::parse_ast_enum_declaration,
        func::{parse_extern_function_declaration, parse_function_declaration},
        import::parse_ast_import_statement,
        matches::parse_match_block,
        structs::{parse_ast_struct_decl_block, parse_ast_struct_declaration},
        values::parse_ast_value,
        vars::parse_ast_variable_declaration,
    },
};

pub mod control;
pub mod ctx;
pub mod enums;
pub mod forms;
pub mod func;
pub mod import;
pub mod lru;
pub mod matches;
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
///	use calsc_ast::ASTContext;
///
/// let mut ast_ctx = ASTContext::new();
///
/// let tokens: Vec<Token> = lexer_tokenize("var s32 test = 5", "test.cal".to_string()).unwrap();
/// let mut ind: usize = 0;
///
/// let node = parse_ast_node_body_member(&tokens, &mut ind, &mut ast_ctx).unwrap();
/// ```
///
pub fn parse_ast_node_body_member(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> Option<DiagResult<ArenaHandle>> {
    match tokens[*ind].kind {
        TokenKind::Var | TokenKind::Mut => Some(parse_ast_variable_declaration(tokens, ind, ctx)),
        TokenKind::For => Some(parse_ast_for_loop(tokens, ind, ctx)),
        TokenKind::Loop => Some(parse_ast_loop(tokens, ind, ctx)),
        TokenKind::While => Some(parse_ast_while_loop(tokens, ind, ctx)),
        TokenKind::If => Some(parse_ast_if_statement(tokens, ind, ctx)),
        TokenKind::Return => Some(parse_ast_return_statement(tokens, ind, ctx)),
        TokenKind::Match => Some(parse_match_block(tokens, ind, ctx)),
        TokenKind::Comment(_) => {
            *ind += 1; // We do not care about body comments
            None
        }
        _ => Some(parse_ast_value(tokens, ind, true, true, true, ctx)),
    }
}

pub fn parse_ast_return_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // return

    let val;

    if tokens[*ind].kind == TokenKind::SemiColon {
        val = None;
        *ind += 1; // ;
    } else {
        val = Some(parse_ast_value(tokens, ind, true, false, true, ctx)?);
    }

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(ASTNodeKind::ReturnStatement { val }, start, end);

    Ok(node.push(ctx))
}

/// Parses an AST body
pub fn parse_ast_body(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<Vec<ArenaHandle>> {
    let mut members: Vec<ArenaHandle> = vec![];

    while tokens[*ind].kind != TokenKind::BraceClose {
        let member = parse_ast_node_body_member(tokens, ind, ctx); // Auto increments

        if member.is_none() {
            continue;
        }

        let member = member.unwrap()?;

        if !ctx.nodes.get(&member).kind.is_body() {
            tokens[*ind].expects(TokenKind::SemiColon)?;
            *ind += 1; // ;
        }

        members.push(member);
    }

    *ind += 1; // }

    Ok(members)
}

/// Parses a top level node
pub fn parse_ast_top_level(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
    comments: &mut CommentContext,
) -> Option<DiagResult<ArenaHandle>> {
    let i = match tokens[*ind].kind {
        TokenKind::Public | TokenKind::Private | TokenKind::Protected => {
            if matches!(
                tokens[*ind + 1].kind,
                TokenKind::Function | TokenKind::ExternFunc | TokenKind::Struct | TokenKind::Enum
            ) {
                *ind + 1
            } else {
                *ind
            }
        }
        _ => *ind,
    };

    match &tokens[i].kind {
        TokenKind::Function => Some(parse_function_declaration(tokens, ind, ctx)),
        TokenKind::ExternFunc => Some(parse_extern_function_declaration(tokens, ind, ctx)),
        TokenKind::Struct => Some(parse_ast_struct_declaration(tokens, ind, ctx)),
        TokenKind::Enum => Some(parse_ast_enum_declaration(tokens, ind, ctx)),
        TokenKind::Decl => Some(parse_ast_struct_decl_block(tokens, ind, ctx)),
        TokenKind::Import => Some(parse_ast_import_statement(tokens, ind, ctx)),
        TokenKind::Module => Some(parse_ast_module(tokens, ind, ctx)),

        TokenKind::Comment(comment) => {
            comments.push(comment.clone());
            *ind += 1;

            None
        }

        _ => {
            return Some(Err(build_unexpected_token_error(
                &tokens[*ind].kind,
                &tokens[*ind],
            )
            .into()));
        }
    }
}

pub fn parse_ast_module(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // module

    let name: HashedString = tokens[*ind].expects_keyword()?.into(); // Auto increments
    let mut body = vec![];

    *ind += 1; // name

    let node;

    if tokens[*ind].kind == TokenKind::BraceOpen {
        *ind += 1; // {

        let mut comments = CommentContext::new();

        let end = tokens[*ind].end.clone();

        while tokens[*ind].kind != TokenKind::BraceClose {
            let res = parse_ast_top_level(tokens, ind, ctx, &mut comments);

            if res.is_none() {
                continue;
            }

            let res = res.unwrap();

            body.push(res?);
        }

        *ind += 1; // }

        node = ASTNode::new(
            ASTNodeKind::Module {
                name,
                body,
                is_bodied: true,
            },
            start,
            end,
        );
    } else {
        let end = tokens[*ind].end.clone();

        node = ASTNode::new(
            ASTNodeKind::Module {
                name,
                is_bodied: false,
                body,
            },
            start,
            end,
        )
    }

    Ok(node.push(ctx))
}

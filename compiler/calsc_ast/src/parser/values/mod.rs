//! Parsing for values. Every parser for values will be contained in that module

use calsc_diagnostics::{
    DiagResult, PosDiagnosticSource, diags::errors::build_unexpected_token_error,
};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaHandle, pos::FilePosition};

use crate::{
    ASTContext,
    parser::{
        forms::parse_element_path_form,
        func::parse_function_call,
        lru::parse_ast_struct_lru,
        utils::peek_ahead,
        values::{
            arrays::{parse_ast_array_init, parse_ast_index_usage},
            binary::{Precedence, parse_ast_binary_operation},
            casting::parse_ast_cast_into,
            conditions::parse_ast_inverse_condition,
            lits::parse_ast_literal,
            ptrs::{parse_ast_pointer_dereference, parse_ast_pointer_reference},
            range::parse_ast_range,
            structs::parse_ast_structured_init,
        },
        vars::{parse_ast_assign, parse_ast_element_reference},
    },
};

pub mod arrays;
pub mod binary;
pub mod casting;
pub mod conditions;
pub mod lits;
pub mod math;
pub mod ptrs;
pub mod range;
pub mod structs;

/// Parses the lexer tokens as an AST value node.
///
/// **This function may only return value-based AST nodes**.
///
/// Aditionally, this function will also try to parse complex expressions such as math expressions if the `parse_post` parameter is true
/// If `invoked_from_body` is true, the nodes returned **may** not be values
///
/// # Errors
/// Returns `Err` if one of the sub parsing functions failed or
/// that the first token given at the index `ind` cannot possibly represent a value
///
/// # Example
/// ```
/// use calsc_lexer::lexer_tokenize;
/// use calsc_ast::parser::values::parse_ast_value;
/// use calsc_ast::nodes::ASTNodeKind;
/// use calsc_ast::ASTContext;
///
/// let mut ind: usize = 0;
/// let mut ast_ctx = ASTContext::new();
/// let tokens = lexer_tokenize("16", "test".to_string()).unwrap();
///
/// let parsed = parse_ast_value(&tokens, &mut ind, true, false, true, &mut ast_ctx).unwrap();
///
/// assert_eq!(ast_ctx.nodes.get(&parsed).kind, ASTNodeKind::IntLiteral(16));
/// ```
pub fn parse_ast_value(
    tokens: &Vec<Token>,
    ind: &mut usize,
    allow_post: bool,
    invoked_from_body: bool, // Used to determine if parse assigns
    allow_ops: bool,         // Used to determine if operations are allowed
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let first = match tokens[*ind].kind {
        TokenKind::IntLiteral(_)
        | TokenKind::FloatLiteral(_)
        | TokenKind::StringLiteral(_)
        | TokenKind::CharLiteral(_)
        | TokenKind::True
        | TokenKind::False => parse_ast_literal(tokens, ind, ctx)?,

        TokenKind::ParenOpen => {
            *ind += 1; // (
            let value = parse_ast_value(tokens, ind, true, false, allow_ops, ctx)?; // Doesn't allow variable assignment inside of parens
            // Auto increments

            tokens[*ind].expects(TokenKind::ParenClose)?;
            *ind += 1; // )

            value
        }

        TokenKind::And => parse_ast_pointer_reference(tokens, ind, ctx)?,
        TokenKind::Star => parse_ast_pointer_dereference(tokens, ind, ctx)?,

        TokenKind::Bang => parse_ast_inverse_condition(tokens, ind, ctx)?,
        TokenKind::BraceOpen => parse_ast_structured_init(tokens, ind, ctx)?,

        TokenKind::BracketOpen => {
            if tokens[*ind + 2].kind == TokenKind::Dot {
                parse_ast_range(tokens, ind, ctx)?
            } else {
                parse_ast_array_init(tokens, ind, ctx)?
            }
        }

        TokenKind::Keyword(_) | TokenKind::Colon => {
            let peeked_ind = peek_ahead(tokens, *ind, |tokens, ind| {
                parse_element_path_form(tokens, ind)
            })
            .1; // Auto increments

            if tokens[peeked_ind].kind == TokenKind::ParenOpen {
                parse_function_call(tokens, ind, ctx)?
            } else {
                parse_ast_element_reference(tokens, ind, ctx)?
            }
        }

        _ => {
            return Err(build_unexpected_token_error(
                &tokens[*ind].kind,
                &PosDiagnosticSource::new(tokens[*ind].start.clone(), tokens[*ind].end.clone()),
            )
            .into());
        }
    };

    let start = ctx.nodes.get(&first).start.clone();

    if allow_post {
        parse_ast_post(tokens, ind, first, start, invoked_from_body, allow_ops, ctx)
    } else {
        Ok(first)
    }
}

pub fn parse_ast_post(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first_node: ArenaHandle,
    start: FilePosition,
    invoked_from_body: bool,
    allow_ops: bool,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let mut modified_node = true;
    let start_two = start.clone();

    let node = match tokens[*ind].kind {
        TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::BackSlash
        | TokenKind::Tilde
        | TokenKind::Question => {
            if !allow_ops {
                modified_node = false;
                first_node
            } else {
                parse_ast_binary_operation(
                    tokens,
                    ind,
                    first_node,
                    start,
                    Precedence::Assignment,
                    ctx,
                )?
            }
        }

        TokenKind::Dot => {
            if tokens[*ind + 1].kind != TokenKind::Dot {
                parse_ast_struct_lru(tokens, ind, first_node, start, ctx)?
            } else {
                modified_node = false;
                first_node
            }
        }

        TokenKind::Bang => {
            if !allow_ops {
                modified_node = false;
                first_node
            } else {
                parse_ast_binary_operation(
                    tokens,
                    ind,
                    first_node,
                    start,
                    Precedence::Assignment,
                    ctx,
                )?
            }
        }

        TokenKind::Equal => {
            if tokens[*ind + 1].kind == TokenKind::Equal {
                parse_ast_binary_operation(
                    tokens,
                    ind,
                    first_node,
                    start,
                    Precedence::Assignment,
                    ctx,
                )?
            } else {
                if invoked_from_body {
                    parse_ast_assign(tokens, ind, first_node, start, ctx)?
                } else {
                    modified_node = false;
                    first_node
                }
            }
        }

        TokenKind::Into => parse_ast_cast_into(tokens, ind, first_node, start, ctx)?,

        TokenKind::AngelBracketOpen | TokenKind::AngelBracketClose => {
            parse_ast_binary_operation(tokens, ind, first_node, start, Precedence::Assignment, ctx)?
        }

        TokenKind::BracketOpen => parse_ast_index_usage(tokens, ind, first_node, start, ctx)?,

        _ => {
            modified_node = false;
            first_node
        }
    };

    if modified_node {
        parse_ast_post(
            tokens,
            ind,
            node,
            start_two,
            invoked_from_body,
            allow_ops,
            ctx,
        )
    } else {
        Ok(node)
    }
}

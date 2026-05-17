//! Parsing for values. Every parser for values will be contained in that module

use std::path::PathBuf;

use calsc_diagnostics::{DiagResult, PosDiagnosticSource, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::pos::FilePosition;

use crate::{
    AST_CONTEXT,
    parser::{
        func::parse_function_call,
        values::{
            conditions::{parse_ast_compare_expression, parse_ast_inverse_condition},
            lits::parse_ast_literal,
            math::parse_ast_math_expression,
            ptrs::{parse_ast_pointer_dereference, parse_ast_pointer_reference},
            range::parse_ast_range,
            structs::parse_ast_structured_init,
        },
        vars::{parse_ast_assign, parse_ast_element_reference},
    },
    refs::ASTArenaReference,
};

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
///
/// let mut ind: usize = 0;
/// let tokens = lexer_tokenize("16", "test".to_string()).unwrap();
///
/// let parsed = parse_ast_value(&tokens, &mut ind, true, false).unwrap();
///
/// assert_eq!(parsed.kind, ASTNodeKind::IntLiteral(16));
/// ```
pub fn parse_ast_value(
    tokens: &Vec<Token>,
    ind: &mut usize,
    allow_post: bool,
    invoked_from_body: bool, // Used to determine if parse assigns
) -> DiagResult<ASTArenaReference> {
    let first = match tokens[*ind].kind {
        TokenKind::IntLiteral(_)
        | TokenKind::FloatLiteral(_)
        | TokenKind::StringLiteral(_)
        | TokenKind::CharLiteral(_)
        | TokenKind::True
        | TokenKind::False => parse_ast_literal(tokens, ind)?,

        TokenKind::ParenOpen => {
            *ind += 1; // (
            let value = parse_ast_value(tokens, ind, true, false)?; // Doesn't allow variable assignment inside of parens
            // Auto increments

            tokens[*ind].expects(TokenKind::ParenClose)?;
            *ind += 1; // )

            value
        }

        TokenKind::And => parse_ast_pointer_reference(tokens, ind)?,
        TokenKind::Star => parse_ast_pointer_dereference(tokens, ind)?,

        TokenKind::Bang => parse_ast_inverse_condition(tokens, ind)?,
        TokenKind::BraceOpen => parse_ast_structured_init(tokens, ind)?,

        TokenKind::BracketOpen => parse_ast_range(tokens, ind)?,

        TokenKind::Keyword(_) => {
            if tokens[*ind + 1].kind == TokenKind::ParenOpen {
                parse_function_call(tokens, ind)?
            } else {
                parse_ast_element_reference(tokens, ind)?
            }
        }

        _ => {
            return Err(build_unexpected_error(
                &tokens[*ind].kind,
                &PosDiagnosticSource::new(tokens[*ind].start.clone(), tokens[*ind].end.clone()),
            )
            .into());
        }
    };

    let mut start = FilePosition::new(PathBuf::new(), 0, 0);
    AST_CONTEXT.with_borrow(|f| start = f.nodes.get(first.clone()).start.clone());

    if allow_post {
        parse_ast_post(tokens, ind, first, start, invoked_from_body)
    } else {
        Ok(first)
    }
}

pub fn parse_ast_post(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first_node: ASTArenaReference,
    start: FilePosition,
    invoked_from_body: bool,
) -> DiagResult<ASTArenaReference> {
    match tokens[*ind].kind {
        TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::BackSlash
        | TokenKind::Tilde
        | TokenKind::Question => return parse_ast_math_expression(tokens, ind, first_node, start),

        TokenKind::Bang => {
            if tokens[*ind].kind == TokenKind::Equal {
                return parse_ast_compare_expression(tokens, ind, first_node, start);
            } else {
                return parse_ast_math_expression(tokens, ind, first_node, start);
            }
        }

        TokenKind::Equal => {
            if tokens[*ind + 1].kind == TokenKind::Equal {
                return parse_ast_compare_expression(tokens, ind, first_node, start);
            } else {
                if invoked_from_body {
                    return parse_ast_assign(tokens, ind, first_node, start);
                } else {
                    return Ok(first_node);
                }
            }
        }

        TokenKind::AngelBracketOpen | TokenKind::AngelBracketClose => {
            parse_ast_compare_expression(tokens, ind, first_node, start)
        }

        _ => Ok(first_node),
    }
}

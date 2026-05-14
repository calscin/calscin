//! Parsing for values. Every parser for values will be contained in that module

use calsc_diagnostics::{DiagResult, PosDiagnosticSource, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    nodes::ASTNode,
    parser::{
        func::parse_function_call, values::lits::parse_ast_literal,
        vars::parse_ast_variable_reference,
    },
};

pub mod lits;

/// Parses the lexer tokens as an AST value node.
///
/// **This function may only return value-based AST nodes**.
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
/// let parsed = parse_ast_value(&tokens, &mut ind).unwrap();
///
/// assert_eq!(parsed.kind, ASTNodeKind::IntLiteral(16));
/// ```
///
pub fn parse_ast_value(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    let first = match tokens[*ind].kind {
        TokenKind::IntLiteral(_)
        | TokenKind::FloatLiteral(_)
        | TokenKind::StringLiteral(_)
        | TokenKind::CharLiteral(_) => parse_ast_literal(tokens, ind)?,

        TokenKind::Keyword(_) => {
            if tokens[*ind].kind == TokenKind::ParenOpen {
                parse_function_call(tokens, ind)?
            } else {
                parse_ast_variable_reference(tokens, ind)?
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

    Ok(first)
}

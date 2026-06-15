use calsc_diagnostics::{DiagPossible, DiagResult};
use calsc_lexer::toks::{Token, TokenKind};

use crate::{ASTContext, parser::parse_ast_top_level};

/// Parses everything into the current [`ASTContext`].
///
/// # Errors
/// This function will error if the parsing failed at any point
///
pub fn parse_ast_whole(tokens: &Vec<Token>) -> DiagResult<ASTContext> {
    let mut ind = 0;
    let mut ctx = ASTContext::new();

    while tokens[ind].kind != TokenKind::Eof {
        let node = parse_ast_top_level(tokens, &mut ind, &mut ctx)?;
        ctx.tree.push(node);
    }

    Ok(ctx)
}

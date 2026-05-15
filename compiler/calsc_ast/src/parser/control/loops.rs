use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::parse_ast_body,
};

#[inline(always)]
pub fn parse_ast_loop(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // loop

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    let body = parse_ast_body(tokens, ind)?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::Loop { body },
        start,
        end,
    )))
}

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{parse_ast_body, values::parse_ast_value},
};

#[inline(always)]
pub fn parse_ast_while_loop(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // while

    tokens[*ind].expects(TokenKind::ParenOpen)?;
    *ind += 1; // (

    let condition_value = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    tokens[*ind].expects(TokenKind::ParenClose)?;
    *ind += 1; // )

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    let body = parse_ast_body(tokens, ind)?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::WhileLoop {
            condition: condition_value,
            body,
        },
        start,
        end,
    )))
}

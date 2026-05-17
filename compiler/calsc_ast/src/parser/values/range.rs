//! Parsing of ranges

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use crate::refs::ASTArenaReference;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::values::parse_ast_value,
};

#[inline(always)]
pub fn parse_ast_range(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    tokens[*ind].expects(TokenKind::BracketOpen)?;
    *ind += 1; // [

    let start_node = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    tokens[*ind].expects(TokenKind::Dot)?;
    *ind += 1; // first .

    tokens[*ind].expects(TokenKind::Dot)?;
    *ind += 1;

    let end_node = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    tokens[*ind].expects(TokenKind::BracketClose)?;
    *ind += 1; // ]

    let mut increment = None;

    if tokens[*ind].kind == TokenKind::Minus {
        *ind += 1; // -

        tokens[*ind].expects(TokenKind::AngelBracketClose)?;
        *ind += 1; // >

        increment = Some(parse_ast_value(tokens, ind, true, false)?); // Auto increments
    }

    let end = tokens[*ind - 1].end.clone(); // Cancels the auto increment to get the end

    let node = ASTNode::new(
        ASTNodeKind::Range {
            start: start_node,
            end: end_node,
            increment,
        },
        start,
        end,
    );

    Ok(node.push())
}

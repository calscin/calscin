use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{parse_ast_body, types::parse_ast_type, values::parse_ast_value},
};

#[inline(always)]
pub fn parse_ast_for_loop(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // for

    let iterator_type = parse_ast_type(tokens, ind)?; // Auto increments

    let iterator_name = tokens[*ind].expects_keyword()?;
    *ind += 1; // keyword

    tokens[*ind].expects(TokenKind::Equal)?;
    *ind += 1; // = 

    tokens[*ind].expects(TokenKind::AngelBracketClose)?;
    *ind += 1; // >

    let value = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    let body = parse_ast_body(tokens, ind)?; // Auto increments

    let end = tokens[*ind - 1].end.clone(); // Removes the auto increment to grab the end

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::ForLoop {
            iterator_type,
            iterator_name: HashedString::new(iterator_name),
            iterated: value,
            body,
        },
        start,
        end,
    )))
}

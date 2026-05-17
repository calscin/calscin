use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaAllocatorReference, hash::HashedString};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{forms::parse_ast_body_form, types::parse_ast_type, values::parse_ast_value},
};

#[inline(always)]
pub fn parse_ast_for_loop(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ArenaAllocatorReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // for

    let iterator_type = parse_ast_type(tokens, ind, true)?; // Auto increments

    let iterator_name = tokens[*ind].expects_keyword()?;
    *ind += 1; // keyword

    tokens[*ind].expects(TokenKind::Equal)?;
    *ind += 1; // = 

    tokens[*ind].expects(TokenKind::AngelBracketClose)?;
    *ind += 1; // >

    let value = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    let body = parse_ast_body_form(tokens, ind)?; // Auto increments

    let end = tokens[*ind - 1].end.clone(); // Removes the auto increment to grab the end

    let node = ASTNode::new(
        ASTNodeKind::ForLoop {
            iterator_type,
            iterator_name: HashedString::new(iterator_name),
            iterated: value,
            body,
        },
        start,
        end,
    );

    Ok(node.push())
}

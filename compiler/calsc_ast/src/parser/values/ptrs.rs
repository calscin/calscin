use crate::refs::ASTArenaReference;
use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::Token;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::values::parse_ast_value,
};

#[inline(always)]
pub fn parse_ast_pointer_reference(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // &

    let value = parse_ast_value(tokens, ind, false, false, true)?; // Doesn't allow post 
    // Auto increments

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(ASTNodeKind::PointerReference(value), start, end);

    Ok(node.push())
}

#[inline(always)]
pub fn parse_ast_pointer_dereference(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // *

    let value = parse_ast_value(tokens, ind, false, false, true)?; // Doesn't allow post
    // Auto increments

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(ASTNodeKind::PointerDereference(value), start, end);

    Ok(node.push())
}

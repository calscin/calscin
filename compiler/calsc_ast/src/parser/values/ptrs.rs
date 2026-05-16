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
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // &

    let value = parse_ast_value(tokens, ind, false, false)?; // Doesn't allow post 
    // Auto increments

    let end = value.end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::PointerReference(value),
        start,
        end,
    )))
}

#[inline(always)]
pub fn parse_ast_pointer_dereference(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // *

    let value = parse_ast_value(tokens, ind, false, false)?; // Doesn't allow post
    // Auto increments

    let end = value.end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::PointerDereference(value),
        start,
        end,
    )))
}

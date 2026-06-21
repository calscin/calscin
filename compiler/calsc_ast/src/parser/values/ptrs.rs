use crate::ASTContext;
use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::values::parse_ast_value,
};

#[inline(always)]
pub fn parse_ast_pointer_reference(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // &

    let mutable = match tokens[*ind].kind {
        TokenKind::Mut => {
            *ind += 1; // mut

            true
        }

        _ => false,
    };

    let value = parse_ast_value(tokens, ind, false, false, true, ctx)?; // Doesn't allow post

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(ASTNodeKind::PointerReference(value, mutable), start, end);

    Ok(node.push(ctx))
}

#[inline(always)]
pub fn parse_ast_pointer_dereference(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // *

    let value = parse_ast_value(tokens, ind, false, false, true, ctx)?; // Doesn't allow post
    // Auto increments

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(ASTNodeKind::PointerDereference(value), start, end);

    Ok(node.push(ctx))
}

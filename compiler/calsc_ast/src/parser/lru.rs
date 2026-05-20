use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::pos::FilePosition;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{func::parse_function_call, vars::parse_ast_element_reference},
    refs::ASTArenaReference,
};

pub(crate) fn parse_ast_struct_lru_member(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ASTArenaReference> {
    tokens[*ind].expects_keyword()?;

    if tokens[*ind].kind == TokenKind::ParenOpen {
        return parse_function_call(tokens, ind); // Auto increments
    } else {
        return parse_ast_element_reference(tokens, ind); // Auto increments
    }
}

/// Parses a AST struct LRU.
/// An AST struct LRU is basically the usage of a field or method on a given token.
pub fn parse_ast_struct_lru(
    tokens: &Vec<Token>,
    ind: &mut usize,
    original: ASTArenaReference,
    start_pos: FilePosition,
) -> DiagResult<ASTArenaReference> {
    *ind += 1; // .

    let right_expr = parse_ast_struct_lru_member(tokens, ind)?; // Auto increments
    let end_pos = tokens[*ind].end.clone();

    let val = ASTNode::new(
        ASTNodeKind::StructLRUsage {
            left_expr: original,
            right_expr,
        },
        start_pos,
        end_pos,
    );

    Ok(val.push())
}

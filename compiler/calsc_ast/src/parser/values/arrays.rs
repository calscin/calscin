use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::pos::FilePosition;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::values::parse_ast_value,
    refs::ASTArenaReference,
};

pub fn parse_ast_index_usage(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first_node: ASTArenaReference,
    start: FilePosition,
) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // [

    let index = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    tokens[*ind].expects(TokenKind::BracketClose)?;

    let end = tokens[*ind].end.clone();

    *ind += 1; // ]

    Ok(ASTNode::new(
        ASTNodeKind::IndexUsage {
            val: first_node,
            index,
        },
        start,
        end,
    )
    .push())
}

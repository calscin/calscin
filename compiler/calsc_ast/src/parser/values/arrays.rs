use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::pos::FilePosition;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{utils::parse_ast_list, values::parse_ast_value},
    refs::ASTArenaReference,
};

pub fn parse_ast_index_usage(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first_node: ASTArenaReference,
    start: FilePosition,
) -> DiagResult<ASTArenaReference> {
    *ind += 1; // [

    let index = parse_ast_value(tokens, ind, true, false, true)?; // Auto increments

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

pub fn parse_ast_array_init(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // [

    let values = parse_ast_list(
        tokens,
        ind,
        &mut |tokens, ind| parse_ast_value(tokens, ind, true, false, true),
        TokenKind::BracketClose,
        false,
        false,
    )?; // Auto increments

    let end = tokens[*ind - 1].end.clone(); // Bypass auto increment to get end

    let node = ASTNode::new(ASTNodeKind::ArrayInit(values), start, end);

    Ok(node.push())
}

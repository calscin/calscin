use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaHandle, pos::FilePosition};

use crate::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
    parser::{utils::parse_ast_list, values::parse_ast_value},
};

pub fn parse_ast_index_usage(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first_node: ArenaHandle,
    start: FilePosition,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    *ind += 1; // [

    let index = parse_ast_value(tokens, ind, true, false, true, ctx)?; // Auto increments

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
    .push(ctx))
}

pub fn parse_ast_array_init(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // [

    let values = parse_ast_list(
        tokens,
        ind,
        &mut |tokens, ind| parse_ast_value(tokens, ind, true, false, true, ctx),
        TokenKind::BracketClose,
        false,
        false,
    )?; // Auto increments

    let end = tokens[*ind - 1].end.clone(); // Bypass auto increment to get end

    let node = ASTNode::new(ASTNodeKind::ArrayInit(values), start, end);

    Ok(node.push(ctx))
}

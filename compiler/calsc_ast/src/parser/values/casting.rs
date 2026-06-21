use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::Token;
use calsc_utils::{alloc::arena::ArenaHandle, pos::FilePosition};

use crate::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
    parser::types::parse_ast_type,
};

pub fn parse_ast_cast_into(
    tokens: &Vec<Token>,
    ind: &mut usize,
    origin: ArenaHandle,
    start_pos: FilePosition,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    *ind += 1; // into

    let ty = parse_ast_type(tokens, ind, true)?; // Auto increments
    let end = tokens[*ind - 1].end.clone(); // Counters the auto increment

    let node = ASTNode::new(
        ASTNodeKind::IntoCast {
            val: origin,
            into: ty,
        },
        start_pos,
        end,
    );

    Ok(node.push(ctx))
}

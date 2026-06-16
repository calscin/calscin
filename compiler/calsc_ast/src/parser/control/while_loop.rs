use crate::ASTContext;
use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::Token;
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::forms::{parse_ast_body_form, parse_ast_condition_form},
};

#[inline(always)]
pub fn parse_ast_while_loop(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // while

    let condition_value = parse_ast_condition_form(tokens, ind, ctx)?; // Auto increments
    let body = parse_ast_body_form(tokens, ind, ctx)?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(
        ASTNodeKind::WhileLoop {
            condition: condition_value,
            body,
        },
        start,
        end,
    );

    Ok(node.push(ctx))
}

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::Token;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::values::parse_ast_value,
};

pub fn parse_ast_inverse_condition(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // !

    let val = parse_ast_value(tokens, ind, false)?; // Does not parse post. The parse_ast_value statement of parse_inverse_condition has priority
    // Auto increments

    let end = tokens[*ind - 1].end.clone(); // Counters the auto increment to get the end

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::InverseCondition(val),
        start,
        end,
    )))
}

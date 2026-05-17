use std::collections::HashMap;

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaAllocatorReference, hash::HashedString};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{utils::parse_ast_list, values::parse_ast_value},
};

pub(crate) fn parse_structured_init_field(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<(HashedString, ArenaAllocatorReference)> {
    let name = tokens[*ind].expects_keyword()?;
    *ind += 1; // keyword

    tokens[*ind].expects(TokenKind::Colon)?;
    *ind += 1; // :

    let value = parse_ast_value(tokens, ind, true, false)?;

    Ok((HashedString::new(name), value))
}

pub fn parse_ast_structured_init(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ArenaAllocatorReference> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // {

    let fields = parse_ast_list(
        tokens,
        ind,
        &mut parse_structured_init_field,
        TokenKind::BraceClose,
        true,
        false,
    )?;

    let end = tokens[*ind - 1].end.clone(); // Bypasses auto incrementation of parse_ast_list to obtain the end

    let mut actual_fields = HashMap::new();

    for field in fields {
        actual_fields.insert(field.0, field.1);
    }

    let node = ASTNode::new(
        ASTNodeKind::StructuredInit {
            values: actual_fields,
        },
        start,
        end,
    );

    Ok(node.push())
}

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{forms::parse_ast_field_form, utils::parse_ast_list},
};

#[inline(always)]
pub fn parse_ast_struct_declaration(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // struct

    let name: HashedString = tokens[*ind].expects_keyword()?.into();
    *ind += 1; // keyword

    let mut type_params = vec![];

    if tokens[*ind].kind == TokenKind::AngelBracketOpen {
        *ind += 1; // <

        type_params = parse_ast_list(
            tokens,
            ind,
            &mut |tokens, ind| Ok(HashedString::from(tokens[*ind].expects_keyword()?)),
            TokenKind::AngelBracketClose,
            true,
            true,
        )?; // Auto increments
    }

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1;

    let fields = parse_ast_list(
        tokens,
        ind,
        &mut parse_ast_field_form,
        TokenKind::BraceClose,
        true,
        false,
    )?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::StructDeclaration {
            name,
            type_params,
            fields,
        },
        start,
        end,
    )))
}

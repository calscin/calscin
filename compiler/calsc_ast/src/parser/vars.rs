//! Parsing related to variables

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{types::parse_type, values::parse_ast_value},
};

#[inline(always)]
pub fn parse_ast_variable_declaration(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    let mutable = match tokens[*ind].kind {
        TokenKind::Mut => true,
        TokenKind::Var => false,
        _ => panic!("Invalid token"),
    };

    *ind += 1; // mut / var

    let ty = parse_type(tokens, ind)?; // Auto increments
    let val;

    let name = HashedString::new(tokens[*ind].expects_keyword()?);
    *ind += 1; // keyword

    if tokens[*ind].kind == TokenKind::Equal {
        *ind += 1; // =

        val = Some(parse_ast_value(tokens, ind)?); // Auto increments
    } else {
        val = None;
    }

    let end = tokens[*ind - 1].end.clone(); // Removes one to remove the auto increment

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::VariableDeclaration {
            mutable,
            var_type: ty,
            name,
            value: val,
        },
        start,
        end,
    )))
}

#[inline]
pub fn parse_variable_reference(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();
    let end = tokens[*ind].end.clone();

    let name = HashedString::new(tokens[*ind].expects_keyword()?);

    *ind += 1; // keyword

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::VariableReference(name),
        start,
        end,
    )))
}

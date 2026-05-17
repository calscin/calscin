//! Parsing related to variables

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{forms::parse_ast_field_form, types::parse_ast_type, values::parse_ast_value},
};

/// Parses a variable declaration
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

    let val;

    let field = parse_ast_field_form(tokens, ind)?;
    let ty = field.0.clone();
    let name = field.1;

    if tokens[*ind].kind == TokenKind::Equal {
        *ind += 1; // =

        val = Some(parse_ast_value(tokens, ind, true, false)?); // Auto increments
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
pub fn parse_ast_element_reference(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();
    let end = tokens[*ind].end.clone();

    let name = HashedString::new(tokens[*ind].expects_keyword()?);

    *ind += 1; // keyword

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::ElementReference(name),
        start,
        end,
    )))
}

#[inline]
pub fn parse_ast_assign(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first: Box<ASTNode>,
) -> DiagResult<Box<ASTNode>> {
    let start = first.start.clone();

    *ind += 1; // =

    let value = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    let end = value.end.clone();

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::Assignment {
            variable: first,
            value,
        },
        start,
        end,
    )))
}

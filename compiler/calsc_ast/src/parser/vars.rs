//! Parsing related to variables

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{hash::HashedString, pos::FilePosition};

use crate::{
    nodes::{ASTNode, ASTNodeKind},
    parser::{forms::parse_ast_field_form, values::parse_ast_value},
    refs::ASTArenaReference,
};

/// Parses a variable declaration
#[inline(always)]
pub fn parse_ast_variable_declaration(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ASTArenaReference> {
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

    let node = ASTNode::new(
        ASTNodeKind::VariableDeclaration {
            mutable,
            var_type: ty,
            name,
            value: val,
        },
        start,
        end,
    );

    Ok(node.push())
}

#[inline]
pub fn parse_ast_element_reference(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ASTArenaReference> {
    let start = tokens[*ind].start.clone();
    let end = tokens[*ind].end.clone();

    let name = HashedString::new(tokens[*ind].expects_keyword()?);

    *ind += 1; // keyword

    let node = ASTNode::new(ASTNodeKind::ElementReference(name), start, end);

    Ok(node.push())
}

#[inline]
pub fn parse_ast_assign(
    tokens: &Vec<Token>,
    ind: &mut usize,
    first: ASTArenaReference,
    start: FilePosition,
) -> DiagResult<ASTArenaReference> {
    *ind += 1; // =

    let value = parse_ast_value(tokens, ind, true, false)?; // Auto increments

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(
        ASTNodeKind::Assignment {
            variable: first,
            value,
        },
        start,
        end,
    );

    Ok(node.push())
}

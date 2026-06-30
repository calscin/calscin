use std::collections::HashMap;

use calsc_diagnostics::{
    DiagPossible, DiagResult, diags::errors::build_enum_entry_already_present,
};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
    parser::{
        forms::{
            parse_ast_field_form, parse_type_parameters_declaration_form, parse_visibility_form,
        },
        utils::parse_ast_list,
    },
    types::ASTType,
};

pub fn parse_ast_enum_declaration(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    let visibility = parse_visibility_form(tokens, ind); // Auto increments

    *ind += 1; // enum

    let name: HashedString = tokens[*ind].expects_keyword()?.into();
    *ind += 1; // name keyword

    let type_parameters = parse_type_parameters_declaration_form(tokens, ind)?; // Auto increments

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    let mut map: HashMap<HashedString, Vec<(ASTType, HashedString)>> = HashMap::new();

    parse_ast_list(
        tokens,
        ind,
        &mut |tokens, ind| parse_ast_enum_entry(tokens, ind, &mut map),
        TokenKind::BraceClose,
        true,
        false,
    )?;

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(
        ASTNodeKind::EnumDeclaration {
            name,
            entries: map,
            visibility,
            type_parameters,
        },
        start,
        end,
    );

    Ok(node.push(ctx))
}

fn parse_ast_enum_entry(
    tokens: &Vec<Token>,
    ind: &mut usize,
    map: &mut HashMap<HashedString, Vec<(ASTType, HashedString)>>,
) -> DiagPossible {
    let name: HashedString = tokens[*ind].expects_keyword()?.into();
    *ind += 1; // name keyword

    if map.contains_key(&name) {
        return Err(build_enum_entry_already_present(&name, &tokens[*ind - 1]).into());
    }

    if tokens[*ind].kind == TokenKind::ParenOpen {
        *ind += 1; // (

        let fields = parse_ast_list(
            tokens,
            ind,
            &mut |tokens, ind| parse_ast_field_form(tokens, ind),
            TokenKind::ParenClose,
            false,
            false,
        )?;

        map.insert(name, fields);
    } else {
        if tokens[*ind].kind != TokenKind::Comma {
            tokens[*ind].expects(TokenKind::BraceClose)?;
        }

        // Do not increment to let parse_ast_list handle it

        map.insert(name, vec![]);
    }

    Ok(())
}

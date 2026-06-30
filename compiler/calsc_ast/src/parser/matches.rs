use std::collections::HashMap;

use calsc_diagnostics::{
    DiagPossible, DiagResult,
    diags::errors::{build_match_already_branch, build_unexpected_token_error},
};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
    parser::{forms::parse_ast_body_form, types::parse_ast_type, values::parse_ast_value},
    types::ASTType,
};

struct MatchBlockParsingCtx {
    branches: HashMap<ASTType, (HashedString, Vec<ArenaHandle>)>,
    default_branch: Option<Vec<ArenaHandle>>,
}

pub fn parse_match_block(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // match

    let mut parse_ctx = MatchBlockParsingCtx {
        branches: HashMap::new(),
        default_branch: None,
    };

    let val = parse_ast_value(tokens, ind, true, false, true, ctx)?; // Auto increments

    tokens[*ind].expects(TokenKind::BraceOpen)?;
    *ind += 1; // {

    while tokens[*ind].kind != TokenKind::BraceClose {
        parse_match_branch(tokens, ind, ctx, &mut parse_ctx)?; // Auto increments
    }

    let end = tokens[*ind].end.clone();

    *ind += 1; // }

    let mut matches = vec![];
    let default_branch = parse_ctx.default_branch.clone();

    for m in parse_ctx.branches {
        matches.push(m)
    }

    let node = ASTNode::new(
        ASTNodeKind::MatchBlock {
            val,
            matches,
            default_match: default_branch,
        },
        start,
        end,
    );

    Ok(node.push(ctx))
}

fn parse_match_branch(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
    parse_ctx: &mut MatchBlockParsingCtx,
) -> DiagPossible {
    match tokens[*ind].kind {
        TokenKind::Underscore => {
            if parse_ctx.default_branch.is_some() {
                return Err(build_unexpected_token_error(&tokens[*ind].kind, &tokens[*ind]).into());
            }

            *ind += 1; // _

            parse_match_arrow(tokens, ind)?; // Auto increments

            let body = parse_ast_body_form(tokens, ind, ctx)?; // Auto increments

            parse_ctx.default_branch = Some(body);

            Ok(())
        }

        _ => {
            let start = *ind;

            let ty = parse_ast_type(tokens, ind, true)?; // Auto increments

            let var_name: HashedString = tokens[*ind].expects_keyword()?.into();
            *ind += 1; // var name keyword

            parse_match_arrow(tokens, ind)?; // Auto increments

            let body = parse_ast_body_form(tokens, ind, ctx)?; // Auto increments

            if parse_ctx.branches.contains_key(&ty) {
                return Err(build_match_already_branch(&ty, &tokens[start]).into());
            }

            parse_ctx.branches.insert(ty, (var_name, body));

            Ok(())
        }
    }
}

pub fn parse_match_arrow(tokens: &Vec<Token>, ind: &mut usize) -> DiagPossible {
    tokens[*ind].expects(TokenKind::Equal)?;
    *ind += 1; // =

    tokens[*ind].expects(TokenKind::AngelBracketClose)?;
    *ind += 1;

    Ok(())
}

use std::collections::HashMap;

use calsc_diagnostics::{DiagPossible, DiagResult};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{ASTContext, parser::values::parse_ast_value, types::ASTType};

struct MatchBlockParsingCtx {
    branches: HashMap<ASTType, Vec<ArenaHandle>>,
    default_branch: Vec<ArenaHandle>,
}

pub fn parse_match_block(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    *ind += 1; // match

    let mut parse_ctx = MatchBlockParsingCtx {
        branches: HashMap::new(),
        default_branch: vec![],
    };

    let val = parse_ast_value(tokens, ind, true, false, true, ctx)?; // Auto increments

    tokens[*ind].expects(TokenKind::BracketOpen)?;
    *ind += 1; // {

    todo!()
}

pub fn parse_match_branch(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
    parse_ctx: &mut MatchBlockParsingCtx,
) -> DiagPossible {
    match tokens[*ind].kind {
        TokenKind::
    }
}

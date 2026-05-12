//! Parsing related to types

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::types::{ASTType, SimpleASTType};

pub fn parse_type(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ASTType> {
    todo!()
}

pub fn parse_type_step(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<SimpleASTType> {
    todo!()
}

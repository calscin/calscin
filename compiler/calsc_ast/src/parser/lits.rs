//! Literal parsing

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::nodes::{ASTNode, ASTNodeKind};

pub fn parse_ast_literal(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();
    let end = tokens[*ind].end.clone();

    let kind = match &tokens[*ind].kind {
        TokenKind::IntLiteral(val) => ASTNodeKind::IntLiteral(*val),
        TokenKind::FloatLiteral(val) => ASTNodeKind::FloatLiteral(*val),
        TokenKind::StringLiteral(val) => ASTNodeKind::StringLiteral(val.clone()),
        TokenKind::CharLiteral(val) => ASTNodeKind::CharLiteral(*val),

        _ => panic!("Invalid node"),
    };

    *ind += 1; // Post increment

    Ok(Box::new(ASTNode::new(kind, start, end)))
}

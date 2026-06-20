use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};

use crate::{ASTContext, parser::parse_ast_top_level};

/// A context that stores current comments for a next element
pub struct CommentContext {
    pub comments: Vec<String>,
}

impl CommentContext {
    pub fn new() -> Self {
        Self { comments: vec![] }
    }

    pub fn push(&mut self, comment: String) {
        self.comments.push(comment);
    }

    pub fn consume(&mut self) -> Vec<String> {
        let vec = self.comments.clone();
        self.comments.clear();

        vec
    }
}

/// Parses everything into the current [`ASTContext`].
///
/// # Errors
/// This function will error if the parsing failed at any point
///
pub fn parse_ast_whole(tokens: &Vec<Token>) -> DiagResult<ASTContext> {
    let mut ind = 0;
    let mut ctx = ASTContext::new();

    while tokens[ind].kind != TokenKind::Eof {
        let node = parse_ast_top_level(tokens, &mut ind, &mut ctx)?;
        ctx.tree.push(node);
    }

    Ok(ctx)
}

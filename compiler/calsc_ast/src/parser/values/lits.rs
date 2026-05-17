//! Literal parsing

use calsc_diagnostics::DiagResult;
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::alloc::arena::ArenaAllocatorReference;

use crate::nodes::{ASTNode, ASTNodeKind};

/// Parses the lexer literal tokens into AST variants.
///
/// # Accepted token types:
/// - `IntLiteral`
/// - `FloatLiteral`
/// - `StringLiteral`
/// - `CharLiteral`
///
/// # Panics
/// This function will panic if tokens with kinds that aren't accepted as being literals are passed
///
/// # Example
/// ```
/// use calsc_lexer::lexer_tokenize;
/// use calsc_ast::parser::values::lits::parse_ast_literal;
/// use calsc_ast::nodes::ASTNodeKind;
///
/// let mut ind: usize = 0;
/// let tokens = lexer_tokenize("16", "test".to_string()).unwrap();
///
/// let parsed = parse_ast_literal(&tokens, &mut ind).unwrap();
///
/// assert_eq!(parsed.kind, ASTNodeKind::IntLiteral(16));
/// ```
///
pub fn parse_ast_literal(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<ArenaAllocatorReference> {
    let start = tokens[*ind].start.clone();
    let end = tokens[*ind].end.clone();

    let kind = match &tokens[*ind].kind {
        TokenKind::IntLiteral(val) => ASTNodeKind::IntLiteral(*val),
        TokenKind::FloatLiteral(val) => ASTNodeKind::FloatLiteral(*val),
        TokenKind::StringLiteral(val) => ASTNodeKind::StringLiteral(val.clone()),
        TokenKind::CharLiteral(val) => ASTNodeKind::CharLiteral(*val),
        TokenKind::True => ASTNodeKind::BooleanLiteral(true),
        TokenKind::False => ASTNodeKind::BooleanLiteral(false),

        _ => panic!("Invalid node"),
    };

    *ind += 1; // Post increment

    let node = ASTNode::new(kind, start, end);
    Ok(node.push())
}

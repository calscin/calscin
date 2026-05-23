use calsc_diagnostics::DiagPossible;
use calsc_lexer::toks::{Token, TokenKind};

use crate::{AST_CONTEXT, parser::parse_ast_top_level};

/// Parses everything into the current [`ASTContext`].
///
/// # Errors
/// This function will error if the parsing failed at any point
///
pub fn parse_ast_whole(tokens: &Vec<Token>) -> DiagPossible {
    let mut ind = 0;

    while tokens[ind].kind != TokenKind::Eof {
        let node = parse_ast_top_level(tokens, &mut ind)?;

        if node.is_additional_tree() {
            AST_CONTEXT.with_borrow_mut(|f| f.additional_tree.push(node))
        } else {
            let name = node.get_top_level_name();

            AST_CONTEXT.with_borrow_mut(|f| {
                f.tree.insert(name.clone(), node);
                f.tree_order.push(name);
            });
        }
    }

    Ok(())
}

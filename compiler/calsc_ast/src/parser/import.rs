use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::hash::HashedString;

use crate::{
    imports::{ImportKind, ImportModule},
    nodes::{ASTNode, ASTNodeKind},
    parser::utils::parse_ast_list,
};

pub fn parse_ast_import_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
) -> DiagResult<Box<ASTNode>> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // import

    let module = match &tokens[*ind].kind {
        TokenKind::Std => ImportModule::Std,
        TokenKind::Keyword(raw) => ImportModule::Package(raw.clone().into()),

        _ => return Err(build_unexpected_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    };

    *ind += 1; // std or keyword

    let mut path: Vec<HashedString> = vec![];

    while tokens[*ind].kind == TokenKind::Colon {
        *ind += 1; // first :

        tokens[*ind].expects(TokenKind::Colon)?;
        *ind += 1; // second :

        path.push(tokens[*ind].expects_keyword()?.into());
        *ind += 1; // keyword
    }

    let mut kind = ImportKind::Whole;

    if tokens[*ind].kind == TokenKind::BracketOpen {
        *ind += 1; // [

        let list = parse_ast_list(
            tokens,
            ind,
            &mut |tokens, ind| Ok(HashedString::from(tokens[*ind].expects_keyword()?)),
            TokenKind::BracketClose,
            true,
            true,
        )?; // Auto increments

        kind = ImportKind::Items(list);
    }

    let end = tokens[*ind - 1].end.clone(); // Cancels the auto increment

    Ok(Box::new(ASTNode::new(
        ASTNodeKind::ImportStatement {
            source: module,
            path,
            kind,
        },
        start,
        end,
    )))
}

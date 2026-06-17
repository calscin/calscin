use calsc_diagnostics::{DiagResult, diags::errors::build_unexpected_token_error};
use calsc_lexer::toks::{Token, TokenKind};
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::{
    ASTContext,
    imports::ImportKind,
    nodes::{ASTNode, ASTNodeKind},
    parser::utils::parse_ast_list,
    path::ElementPath,
};

pub fn parse_ast_import_path(tokens: &Vec<Token>, ind: &mut usize) -> DiagResult<ElementPath> {
    let mut relative = false;
    let mut path = vec![];

    let first: HashedString = match &tokens[*ind].kind {
        TokenKind::Keyword(inner) => inner.clone().into(),
        TokenKind::Colon => {
            *ind += 1; // first :

            tokens[*ind].expects(TokenKind::Colon)?;
            *ind += 1; // second :

            relative = true;

            tokens[*ind].expects_keyword()?.into()
        }

        _ => return Err(build_unexpected_token_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    };

    *ind += 1; // keyword

    path.push(first);

    while tokens[*ind].kind == TokenKind::Colon {
        *ind += 1; // first :

        tokens[*ind].expects(TokenKind::Colon)?;
        *ind += 1; // second :

        if tokens[*ind].kind == TokenKind::BracketOpen
            || tokens[*ind].kind == TokenKind::Star
            || tokens[*ind].kind == TokenKind::SemiColon
        {
            break;
        }

        let val = tokens[*ind].expects_keyword()?;
        *ind += 1; // keyword

        path.push(val.into());
    }

    Ok(ElementPath {
        relative,
        members: path,
    })
}

pub fn parse_ast_import_statement(
    tokens: &Vec<Token>,
    ind: &mut usize,
    ctx: &mut ASTContext,
) -> DiagResult<ArenaHandle> {
    let start = tokens[*ind].start.clone();

    *ind += 1; // import

    let import_path = parse_ast_import_path(tokens, ind)?; // Auto increments

    let kind = match tokens[*ind].kind {
        TokenKind::Star => {
            *ind += 1; // *

            ImportKind::Whole
        }

        TokenKind::SemiColon => {
            *ind += 1; // ;

            ImportKind::Module
        }

        TokenKind::BracketOpen => {
            *ind += 1; // [

            let list = parse_ast_list(
                tokens,
                ind,
                &mut |tokens, ind| tokens[*ind].expects_keyword(),
                TokenKind::BracketClose,
                true,
                true,
            )?; // Auto increments

            ImportKind::Items(list.iter().map(|elem| elem.clone().into()).collect())
        }

        _ => return Err(build_unexpected_token_error(&tokens[*ind].kind, &tokens[*ind]).into()),
    };

    //*ind += 1;

    let end = tokens[*ind - 1].end.clone();

    let node = ASTNode::new(
        ASTNodeKind::ImportStatement {
            path: import_path,
            kind,
        },
        start,
        end,
    );

    Ok(node.push(ctx))
}

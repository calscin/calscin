//! Tests for the AST part of Calscin. Mostly is composed of parser tests

#[cfg(test)]
use calsc_ast::ASTContext;

#[cfg(test)]
use calsc_ast::{
    imports::ImportKind, nodes::ASTNodeKind, parser::import::parse_ast_import_statement,
};

#[cfg(test)]
use calsc_ast::path::ElementPath;

#[cfg(test)]
use calsc_diagnostics::result::CalscinResult;

#[cfg(test)]
use calsc_lexer::lexer_tokenize;

pub mod control;
pub mod func;
pub mod types;
pub mod vals;
pub mod vars;

#[test]
fn parse_import_statement_whole_test() {
    let mut ctx = ASTContext::new();
    let tokens = lexer_tokenize("import std::meow::*", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let import = parse_ast_import_statement(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let import_ref = ctx.nodes.get(&import);

    if let ASTNodeKind::ImportStatement { path, kind } = import_ref.kind.clone() {
        assert_eq!(path, ElementPath::new(vec!["std".into(), "meow".into()]));
        assert_eq!(kind, ImportKind::Whole);
    } else {
        panic!()
    }
}

#[test]
fn parse_import_statement_elements_test() {
    let mut ctx = ASTContext::new();
    let tokens =
        lexer_tokenize("import meow::test[print]", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let import = parse_ast_import_statement(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let import_ref = ctx.nodes.get(&import);

    if let ASTNodeKind::ImportStatement { path, kind } = import_ref.kind.clone() {
        assert_eq!(path, ElementPath::new(vec!["meow".into(), "test".into()]));
        assert_eq!(kind, ImportKind::Items(vec!["print".into()]));
    } else {
        panic!()
    }
}

#[test]
pub fn parse_import_statement_module_test() {
    let mut ctx = ASTContext::new();
    let tokens = lexer_tokenize("import meow::test;", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let import = parse_ast_import_statement(&tokens, &mut ind, &mut ctx).unwrap_cleanly();
    let import_ref = ctx.nodes.get(&import);

    if let ASTNodeKind::ImportStatement { path, kind } = import_ref.kind.clone() {
        assert_eq!(path, ElementPath::new(vec!["meow".into(), "test".into()]));
        assert_eq!(kind, ImportKind::Module);
    }
}

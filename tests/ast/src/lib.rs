//! Tests for the AST part of Calscin. Mostly is composed of parser tests

use calsc_ast::{
    imports::{ImportKind, ImportModule},
    nodes::ASTNodeKind,
    parser::import::parse_ast_import_statement,
};
use calsc_diagnostics::result::CalscinResult;
use calsc_lexer::lexer_tokenize;

pub mod control;
pub mod func;
pub mod types;
pub mod vals;
pub mod vars;

#[test]
fn parse_import_statement_whole_test() {
    let tokens = lexer_tokenize("import std::meow", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let import = parse_ast_import_statement(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::ImportStatement { source, path, kind } = import.kind.clone() {
        assert_eq!(source, ImportModule::Std);
        assert_eq!(path, vec!["meow".into()]);
        assert_eq!(kind, ImportKind::Whole);
    } else {
        panic!()
    }
}

#[test]
fn parse_import_statement_elements_test() {
    let tokens =
        lexer_tokenize("import meow::test[print]", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let import = parse_ast_import_statement(&tokens, &mut ind).unwrap_cleanly();

    if let ASTNodeKind::ImportStatement { source, path, kind } = import.kind.clone() {
        assert_eq!(source, ImportModule::Package("meow".into()));
        assert_eq!(path, vec!["test".into()]);
        assert_eq!(kind, ImportKind::Items(vec!["print".into()]));
    } else {
        panic!()
    }
}

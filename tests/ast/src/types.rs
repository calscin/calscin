//! Parsing tests related to type parsing

#[cfg(test)]
use calsc_ast::{parser::types::parse_ast_type, types::ASTType};

#[cfg(test)]
use calsc_diagnostics::result::CalscinResult;

#[cfg(test)]
use calsc_lexer::lexer_tokenize;

#[test]
pub fn test_simple_type_parsing() {
    let tokens = lexer_tokenize("s32", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let ty = parse_ast_type(&tokens, &mut ind, true).unwrap_cleanly();

    assert_eq!(ty, ASTType::Generic("s32".into(), None, vec![]));
}

#[test]
pub fn test_simple_type_parsing_generic_type_specs() {
    let tokens = lexer_tokenize("s32<test, abcdef>", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let ty = parse_ast_type(&tokens, &mut ind, true).unwrap_cleanly();

    assert_eq!(
        ty,
        ASTType::Generic(
            "s32".into(),
            None,
            vec!["test".to_string(), "abcdef".to_string()]
        )
    )
}

#[test]
pub fn test_simple_type_parsing_size_spec() {
    let tokens = lexer_tokenize("s.32", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let ty = parse_ast_type(&tokens, &mut ind, true).unwrap_cleanly();

    assert_eq!(ty, ASTType::Generic("s".into(), Some(32), vec![]))
}

#[test]
pub fn test_complex_type_parsing() {
    let tokens = lexer_tokenize("s.32<test, abcdef>[32]&", "test.col".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let ty = parse_ast_type(&tokens, &mut ind, true).unwrap_cleanly();

    assert_eq!(
        ty,
        ASTType::Reference(
            false,
            Box::new(ASTType::Array(
                32,
                Box::new(ASTType::Generic(
                    "s".into(),
                    Some(32),
                    vec!["test".to_string(), "abcdef".to_string()]
                ))
            ))
        )
    )
}

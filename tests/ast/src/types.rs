//! Parsing tests related to type parsing

use calsc_ast::path::ElementPath;
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

    assert_eq!(
        ty,
        ASTType::Generic(ElementPath::new_relative(vec!["s32".into()]), None, vec![])
    );
}

#[test]
pub fn test_simple_type_parsing_generic_type_specs() {
    let tokens = lexer_tokenize("s32<test, abcdef>", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let ty = parse_ast_type(&tokens, &mut ind, true).unwrap_cleanly();

    let type1 = lexer_tokenize("test", "test.cal".to_string()).unwrap_cleanly();
    let type2 = lexer_tokenize("abcdef", "test.cal".to_string()).unwrap_cleanly();

    let mut ind1 = 0;
    let mut ind2 = 0;

    assert_eq!(
        ty,
        ASTType::Generic(
            ElementPath::new_relative(vec!["s32".into()]),
            None,
            vec![
                parse_ast_type(&type1, &mut ind1, true).unwrap_cleanly(),
                parse_ast_type(&type2, &mut ind2, true).unwrap_cleanly()
            ]
        )
    )
}

#[test]
pub fn test_simple_type_parsing_size_spec() {
    let tokens = lexer_tokenize("s.32", "test.cal".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let ty = parse_ast_type(&tokens, &mut ind, true).unwrap_cleanly();

    assert_eq!(
        ty,
        ASTType::Generic(
            ElementPath::new_relative(vec!["s".into()]),
            Some(32),
            vec![]
        )
    )
}

#[test]
pub fn test_complex_type_parsing() {
    let tokens = lexer_tokenize("s.32<test, abcdef>[32]&", "test.col".to_string()).unwrap_cleanly();
    let mut ind = 0;

    let ty = parse_ast_type(&tokens, &mut ind, true).unwrap_cleanly();

    let type1 = lexer_tokenize("test", "test.cal".to_string()).unwrap_cleanly();
    let type2 = lexer_tokenize("abcdef", "test.cal".to_string()).unwrap_cleanly();

    let mut ind1 = 0;
    let mut ind2 = 0;

    assert_eq!(
        ty,
        ASTType::Reference(
            true,
            Box::new(ASTType::Array(
                Some(32),
                Box::new(ASTType::Generic(
                    ElementPath::new_relative(vec!["s".into()]),
                    Some(32),
                    vec![
                        parse_ast_type(&type1, &mut ind1, true).unwrap_cleanly(),
                        parse_ast_type(&type2, &mut ind2, true).unwrap_cleanly()
                    ]
                ))
            ))
        )
    )
}

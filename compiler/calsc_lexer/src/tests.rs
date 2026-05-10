use crate::{lexer_tokenize, toks::TokenKind};

#[test]
fn parse_float_token() {
    let tokens = lexer_tokenize("2.5844744", "test".to_string()).unwrap();

    assert!(tokens[0].is_float_lit());
    assert_eq!(tokens[0].kind, TokenKind::FloatLiteral(2.5844744));
}

#[test]
fn parse_int_token() {
    let tokens = lexer_tokenize("123456", "test".to_string()).unwrap();

    assert!(tokens[0].is_int_lit());
    assert_eq!(tokens[0].kind, TokenKind::IntLiteral(123456));
}

#[test]
fn parse_int_float_token() {
    let tokens = lexer_tokenize("1.123 456", "test".to_string()).unwrap();

    assert!(tokens[0].is_float_lit());
    assert!(tokens[1].is_int_lit());
    assert_eq!(tokens[0].kind, TokenKind::FloatLiteral(1.123));
    assert_eq!(tokens[1].kind, TokenKind::IntLiteral(456));
}

#[test]
fn parse_keyword_token() {
    let tokens = lexer_tokenize("abcdef", "test".to_string()).unwrap();

    assert!(tokens[0].is_keyword());
    assert_eq!(tokens[0].kind, TokenKind::Keyword("abcdef".to_string()));
}

#[test]
fn parse_hashed_keyword_token() {
    let tokens = lexer_tokenize("func", "test".to_string()).unwrap();

    assert!(tokens[0].kind == TokenKind::Function);
}

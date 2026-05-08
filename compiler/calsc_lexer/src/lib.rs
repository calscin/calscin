//! The Calscin lexer. Is used at the start of the compilation process to transform raw text into lexer tokens
//! so that the AST can use it.

#![deny(unstable_features)]

use std::path::PathBuf;

use calsc_utils::pos::FilePosition;

use crate::toks::{Token, TokenKind};

pub mod toks;

/// Converts raw texts into lexer tokens.
/// # Example
/// ```
/// use calsc_lexer::lexer_tokenize;
/// use calsc_lexer::toks::Token;
///
/// let tokens: Vec<Token> = lexer_tokenize("3.14", "test.cl".to_string());
/// ```
pub fn lexer_tokenize(content: &str, file_path: String) -> Vec<Token> {
    let mut tokens = vec![];

    let mut i = 0;
    let mut pos = FilePosition::new(PathBuf::from(file_path), 0, 0);

    while i < content.len() {
        let c = match content.chars().nth(i) {
            Some(v) => v,
            None => break,
        };

        if c == '\n' {
            i += 1;

            pos.new_line();
            continue;
        }

        if c.is_numeric() {
            tokens.push(parse_number_token(content, &mut i, &pos));
        }

        if c == '-' {
            if content.chars().nth(i).is_some() && content.chars().nth(i).unwrap().is_numeric() {
                tokens.push(parse_number_token(content, &mut i, &pos))
            }
        }
    }

    tokens
}

/// Parses the given string at the given position as an number literal token.
/// The result token can either be a float literal or an integer literal.
pub fn parse_number_token(content: &str, ind: &mut usize, start_pos: &FilePosition) -> Token {
    let start = *ind;

    let mut met_dot = false;

    while *ind < content.len() {
        let c = match content.chars().nth(*ind) {
            Some(v) => v,
            None => break,
        };

        if !c.is_numeric() && c != '.' {
            break;
        }

        if c == '.' {
            if met_dot {
                break;
            }

            met_dot = true;
        }

        *ind += 1;
    }

    let end = *ind;
    let end_pos = FilePosition::step_col(&start_pos, end - start);

    let slice = content[start..end].to_string();

    *ind += 1; // Increment to increment i post function usage

    if met_dot {
        let lit: f64 = match slice.parse() {
            Ok(v) => v,
            Err(_) => panic!("Cannot parse float literal"),
        };

        Token::new(TokenKind::FloatLiteral(lit), end_pos)
    } else {
        let lit: i128 = match slice.parse() {
            Ok(v) => v,
            Err(_) => panic!("Cannot parse int literal"),
        };

        Token::new(TokenKind::IntLiteral(lit), end_pos)
    }
}

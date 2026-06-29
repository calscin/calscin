//! The Calscin lexer. Is used at the start of the compilation process to transform raw text into lexer tokens
//! so that the AST can use it.

#![deny(unstable_features)]
#![deny(unsafe_code)]

use std::path::PathBuf;

use calsc_diagnostics::{DiagResult, PosDiagnosticSource, diags::errors::build_cannot_parse_error};
use calsc_utils::{fnvhash, pos::FilePosition};
use unescape::unescape;

use crate::toks::{Token, TokenKind};
use calsc_utils::hash::hash_fnv_1a;

pub mod toks;

pub const FUNCTION_HASH: u64 = fnvhash!("func");
pub const TRUE_HASH: u64 = fnvhash!("true");
pub const FALSE_HASH: u64 = fnvhash!("false");
pub const IF_HASH: u64 = fnvhash!("if");
pub const ELSE_HASH: u64 = fnvhash!("else");
pub const EXTERNFUNC_HASH: u64 = fnvhash!("externfunc");
pub const IMPORT_HASH: u64 = fnvhash!("import");
pub const VAR_HASH: u64 = fnvhash!("var");
pub const MUT_HASH: u64 = fnvhash!("mut");
pub const STRUCT_HASH: u64 = fnvhash!("struct");
pub const DECL_HASH: u64 = fnvhash!("decl");
pub const RETURN_HASH: u64 = fnvhash!("return");
pub const FOR_HASH: u64 = fnvhash!("for");
pub const LOOP_HASH: u64 = fnvhash!("loop");
pub const WHILE_HASH: u64 = fnvhash!("while");
pub const MODULE_HASH: u64 = fnvhash!("module");
pub const PUB_HASH: u64 = fnvhash!("pub");
pub const PROT_HASH: u64 = fnvhash!("prot");
pub const PRIV_HASH: u64 = fnvhash!("priv");
pub const INTO_HASH: u64 = fnvhash!("into");
pub const MATCH_HASH: u64 = fnvhash!("match");

/// Converts raw texts into lexer tokens.
/// # Examples
/// ```
/// use calsc_lexer::lexer_tokenize;
/// use calsc_lexer::toks::Token;
/// use calsc_diagnostics::result::CalscinResult;
///
/// let tokens: Vec<Token> = lexer_tokenize("3.14", "test.cal".to_string()).unwrap_cleanly();
/// assert!(tokens[0].is_float_lit());
///
/// ```
pub fn lexer_tokenize(content: &str, file_path: String) -> DiagResult<Vec<Token>> {
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

        if c == '"' {
            tokens.push(parse_string_token(content, &mut i, &mut pos)?);
            continue;
        }

        if c == '\'' {
            tokens.push(parse_char_token(content, &mut i, &mut pos)?);
            continue;
        }

        if c.is_numeric() {
            tokens.push(parse_number_token(content, &mut i, &mut pos)?);
            continue;
        }

        if c.is_alphabetic() || c == '_' {
            if content.chars().nth(i + 1).unwrap().is_alphabetic() {
                tokens.push(parse_keyword(content, &mut i, &mut pos)?);
            } else {
                tokens.push(Token::new(
                    TokenKind::Underscore,
                    pos.clone(),
                    pos.step_col(1),
                ))
            }
            continue;
        }

        let kind = match c {
            ';' => TokenKind::SemiColon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '(' => TokenKind::ParenOpen,
            ')' => TokenKind::ParenClose,
            '{' => TokenKind::BraceOpen,
            '}' => TokenKind::BraceClose,
            '[' => TokenKind::BracketOpen,
            ']' => TokenKind::BracketClose,
            '@' => TokenKind::At,
            '#' => TokenKind::Pound,
            '~' => TokenKind::Tilde,
            '?' => TokenKind::Question,
            ':' => TokenKind::Colon,
            '!' => TokenKind::Bang,
            '<' => TokenKind::AngelBracketOpen,
            '>' => TokenKind::AngelBracketClose,
            '-' => TokenKind::Minus,
            '&' => TokenKind::And,
            '|' => TokenKind::Or,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '/' => {
                match content.chars().nth(i + 1) {
                    Some(v) => {
                        if v == '/' {
                            tokens.push(parse_comment(content, &mut i, &mut pos)?);
                            continue;
                        }
                    }

                    None => {}
                };

                TokenKind::Slash
            }

            '\\' => TokenKind::BackSlash,
            '^' => TokenKind::Caret,
            '%' => TokenKind::Percent,
            '\n' => TokenKind::Newline,
            '=' => TokenKind::Equal,

            '\t' | ' ' => {
                i += 1;
                pos = pos.step_col(1);

                continue;
            }

            _ => TokenKind::Unknown,
        };

        let end_pos = pos.step_col(1);
        let token = Token::new(kind, pos.clone(), end_pos.clone());

        tokens.push(token);
        i += 1;

        pos = end_pos;
    }

    tokens.push(Token::new(TokenKind::Eof, pos.clone(), pos));

    Ok(tokens)
}

pub fn parse_keyword(
    content: &str,
    ind: &mut usize,
    start_pos: &mut FilePosition,
) -> DiagResult<Token> {
    let start = *ind;

    while *ind < content.len() {
        let c = match content.chars().nth(*ind) {
            Some(v) => v,
            None => break,
        };

        if !c.is_alphanumeric() && c != '_' {
            break;
        }

        *ind += 1;
    }

    let end = *ind;
    let end_pos = FilePosition::step_col(&start_pos, end - start);

    let slice = content[start..end].to_string();

    let kind = match fnvhash!(&slice) {
        FUNCTION_HASH => TokenKind::Function,
        TRUE_HASH => TokenKind::True,
        FALSE_HASH => TokenKind::False,
        IF_HASH => TokenKind::If,
        ELSE_HASH => TokenKind::Else,
        EXTERNFUNC_HASH => TokenKind::ExternFunc,
        IMPORT_HASH => TokenKind::Import,
        VAR_HASH => TokenKind::Var,
        MUT_HASH => TokenKind::Mut,
        STRUCT_HASH => TokenKind::Struct,
        DECL_HASH => TokenKind::Decl,
        RETURN_HASH => TokenKind::Return,
        FOR_HASH => TokenKind::For,
        LOOP_HASH => TokenKind::Loop,
        WHILE_HASH => TokenKind::While,
        MODULE_HASH => TokenKind::Module,
        PUB_HASH => TokenKind::Public,
        PROT_HASH => TokenKind::Protected,
        PRIV_HASH => TokenKind::Private,
        INTO_HASH => TokenKind::Into,
        MATCH_HASH => TokenKind::Match,
        _ => TokenKind::Keyword(slice),
    };

    let res = Ok(Token::new(kind, start_pos.clone(), end_pos.clone()));

    *start_pos = end_pos.step_col(1);

    res
}

/// Parses a comment
pub fn parse_comment(
    content: &str,
    ind: &mut usize,
    start_pos: &mut FilePosition,
) -> DiagResult<Token> {
    let start = *ind;

    *ind += 2; // first / + second /

    while *ind < content.len() {
        let c = match content.chars().nth(*ind) {
            Some(v) => v,
            None => break,
        };

        if c == '\n' {
            break;
        }

        *ind += 1;
    }

    let end = *ind;
    let end_pos = FilePosition::step_col(&start_pos, end - start);

    let slice = content[start..end].to_string();

    let res = Token::new(TokenKind::Comment(slice), start_pos.clone(), end_pos);

    Ok(res)
}

/// Parses the given string at the given position as an number literal token.
/// The result token can either be a float literal or an integer literal.
pub fn parse_number_token(
    content: &str,
    ind: &mut usize,
    start_pos: &mut FilePosition,
) -> DiagResult<Token> {
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
            let next = content.chars().nth(*ind + 1);
            if next.is_some() && next.unwrap() == '.' {
                break; // Range
            }

            if met_dot {
                let end = *ind;
                let end_pos = FilePosition::step_col(&start_pos, end - start);

                let source = PosDiagnosticSource::new(start_pos.clone(), end_pos);

                return Err(build_cannot_parse_error(&"float literal".to_string(), &source).into());
            }

            met_dot = true;
        }

        *ind += 1;
    }

    let end = *ind;
    let end_pos = FilePosition::step_col(&start_pos, end - start);

    let source = PosDiagnosticSource::new(start_pos.clone(), end_pos.clone());

    let slice = content[start..end].to_string();

    if met_dot {
        let lit: f64 = match slice.parse() {
            Ok(v) => v,
            Err(_) => {
                return Err(build_cannot_parse_error(&"float literal".to_string(), &source).into());
            }
        };

        Ok(Token::new(
            TokenKind::FloatLiteral(lit),
            start_pos.clone(),
            end_pos,
        ))
    } else {
        let lit: i128 = match slice.parse() {
            Ok(v) => v,
            Err(_) => {
                return Err(build_cannot_parse_error(&"int literal".to_string(), &source).into());
            }
        };

        let res = Ok(Token::new(
            TokenKind::IntLiteral(lit),
            start_pos.clone(),
            end_pos.clone(),
        ));

        *start_pos = end_pos.step_col(1);

        res
    }
}

/// Parses the raw text at the given position in the given string as a string literal.
pub fn parse_string_token(
    content: &str,
    ind: &mut usize,
    start_pos: &mut FilePosition,
) -> DiagResult<Token> {
    *ind += 1; // Increment to skip the first "

    let start = *ind;
    let mut closed = false;

    while *ind < content.len() {
        let c = match content.chars().nth(*ind) {
            Some(v) => v,
            None => break,
        };

        if c == '"' {
            closed = true;
            break;
        }

        *ind += 1;
    }

    let end = *ind;
    let end_pos = start_pos.step_col(end - start);

    if !closed {
        return Err(build_cannot_parse_error(
            &"string literal".to_string(),
            &PosDiagnosticSource::new(start_pos.clone(), end_pos.clone()),
        )
        .into());
    }

    let slice = unescape(&content[start..end]);

    let slice = match slice {
        Some(v) => v,
        None => {
            return Err(build_cannot_parse_error(
                &"string literal".to_string(),
                &PosDiagnosticSource::new(start_pos.clone(), end_pos.clone()),
            )
            .into());
        }
    };

    *ind += 1;

    let res = Ok(Token::new(
        TokenKind::StringLiteral(slice),
        start_pos.clone(),
        end_pos.clone(),
    ));

    *start_pos = end_pos.step_col(1);

    res
}

pub fn parse_char_token(
    content: &str,
    ind: &mut usize,
    start_pos: &mut FilePosition,
) -> DiagResult<Token> {
    *ind += 1;

    let start = *ind;
    let mut closed = false;

    while *ind < content.len() {
        let c = match content.chars().nth(*ind) {
            Some(v) => v,
            None => break,
        };

        if c == '\'' {
            closed = true;
            break;
        }

        *ind += 1;
    }

    let end = *ind;
    let end_pos = start_pos.step_col(end - start);

    let source = PosDiagnosticSource::new(start_pos.clone(), end_pos.clone());

    if !closed {
        return Err(build_cannot_parse_error(&"char literal".to_string(), &source).into());
    }

    let slice = &content[start..end];

    *ind += 1;

    let lit: char = match slice.parse() {
        Ok(v) => v,
        Err(_) => {
            return Err(build_cannot_parse_error(&"char literal".to_string(), &source).into());
        }
    };

    let res = Ok(Token::new(
        TokenKind::CharLiteral(lit),
        start_pos.clone(),
        end_pos.clone(),
    ));

    *start_pos = end_pos.step_col(1);

    res
}

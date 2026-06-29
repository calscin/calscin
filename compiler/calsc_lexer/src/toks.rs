//! Lexer token related definitions.

use std::fmt::Display;

use calsc_diagnostics::{
    DiagPossible, DiagResult, Diagnostic, DiagnosticCode, DiagnosticSource,
    diags::errors::build_expected_token_error,
    span::{Span, SpanKind},
};
use calsc_utils::pos::FilePosition;

/// A parsed token by the lexer.
/// Contains the kind of token as well as the data such as literal values.
/// Also contains a position of where the token starts and ends
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Token {
    /// The kind of token. Holds the data such as literal values and the overall type of token
    pub kind: TokenKind,

    /// The start position of the token inside of the file
    pub start: FilePosition,

    /// The end position of the token inside of the file
    pub end: FilePosition,
}

/// Enum representing common lexer token kinds.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    /// `func`
    Function,

    /// `true`
    True,

    /// `false`
    False,

    /// `if`
    If,

    /// `loop`
    Loop,

    /// `else`
    Else,

    /// `externfunc`
    ExternFunc,

    /// `import`
    Import,

    /// `var`
    Var,

    /// `mut`
    Mut,

    /// `struct`
    Struct,

    /// `decl`
    Decl,

    /// `return`
    Return,

    /// `for`
    For,

    /// `while`
    While,

    /// `module`
    Module,

    /// `pub`
    Public,

    /// `prot`
    Protected,

    /// `priv`
    Private,

    /// `into`
    Into,

    /// `match`
    Match,

    /// ;
    SemiColon,

    /// ,
    Comma,

    /// .
    Dot,

    /// (
    ParenOpen,

    /// )
    ParenClose,

    /// {
    BraceOpen,

    /// }
    BraceClose,

    /// [
    BracketOpen,

    /// ]
    BracketClose,

    /// @
    At,

    /// #
    Pound,

    /// ~
    Tilde,

    /// ?
    Question,

    /// :
    Colon,

    /// !
    Bang,

    /// <
    AngelBracketOpen,

    /// >
    AngelBracketClose,

    /// -
    Minus,

    /// &
    And,

    /// |
    Or,

    /// +
    Plus,

    /// *
    Star,

    /// /
    Slash,

    /// \
    BackSlash,

    /// ^
    Caret,

    /// %
    Percent,

    /// \n
    Newline,

    /// =
    Equal,

    /// `_`
    Underscore,

    /// A keyword literal (eg: `myTestKeyword`)
    Keyword(String),

    /// A string literal (eg: `"hello"`)
    StringLiteral(String),

    /// A char literal (eg: `'c'`)
    CharLiteral(char),

    /// An integer literal (eg: `11`)
    IntLiteral(i128),

    /// A float literal (eg: `3.14`)
    FloatLiteral(f64),

    Comment(String),

    /// The end of file
    Eof,

    /// An unknown token type
    Unknown,
}

impl Token {
    /// Creates a new lexer token.
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::toks::{TokenKind, Token};
    /// use calsc_utils::pos::FilePosition;
    /// use std::path::PathBuf;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let tok: Token = Token::new(TokenKind::Eof, pos.clone(), pos);
    /// ```
    pub fn new(kind: TokenKind, start: FilePosition, end: FilePosition) -> Self {
        Self { kind, start, end }
    }

    /// Checks if the token is a keyword.
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::toks::{TokenKind, Token};
    /// use calsc_utils::pos::FilePosition;
    /// use std::path::PathBuf;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let tok: Token = Token::new(TokenKind::Keyword("test".to_string()), pos.clone(), pos);
    ///
    /// assert!(tok.is_keyword());
    /// assert!(!tok.is_string_lit());
    ///
    /// ```
    pub fn is_keyword(&self) -> bool {
        match self.kind {
            TokenKind::Keyword(_) => true,
            _ => false,
        }
    }

    /// Checks if the token is a string literal.
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::toks::{TokenKind, Token};
    /// use calsc_utils::pos::FilePosition;
    /// use std::path::PathBuf;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let tok: Token = Token::new(TokenKind::StringLiteral("test".to_string()), pos.clone(), pos);
    ///
    /// assert!(tok.is_string_lit());
    /// assert!(!tok.is_keyword());
    ///
    /// ```
    pub fn is_string_lit(&self) -> bool {
        match self.kind {
            TokenKind::StringLiteral(_) => true,
            _ => false,
        }
    }

    /// Checks if the token is a char literal.
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::toks::{TokenKind, Token};
    /// use calsc_utils::pos::FilePosition;
    /// use std::path::PathBuf;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let tok: Token = Token::new(TokenKind::CharLiteral('e'), pos.clone(), pos);
    ///
    /// assert!(tok.is_char_lit());
    /// assert!(!tok.is_keyword());
    ///
    /// ```
    pub fn is_char_lit(&self) -> bool {
        match self.kind {
            TokenKind::CharLiteral(_) => true,
            _ => false,
        }
    }

    /// Checks if the token is a int literal.
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::toks::{TokenKind, Token};
    /// use calsc_utils::pos::FilePosition;
    /// use std::path::PathBuf;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let tok: Token = Token::new(TokenKind::IntLiteral(123), pos.clone(), pos);
    ///
    /// assert!(tok.is_int_lit());
    /// assert!(!tok.is_float_lit());
    ///
    pub fn is_int_lit(&self) -> bool {
        match self.kind {
            TokenKind::IntLiteral(_) => true,
            _ => false,
        }
    }

    /// Checks if the token is a int literal.
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::toks::{TokenKind, Token};
    /// use calsc_utils::pos::FilePosition;
    /// use std::path::PathBuf;
    ///
    /// let pos: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let tok: Token = Token::new(TokenKind::FloatLiteral(123.0), pos.clone(), pos);
    ///
    /// assert!(tok.is_float_lit());
    /// assert!(!tok.is_int_lit());
    ///
    pub fn is_float_lit(&self) -> bool {
        match self.kind {
            TokenKind::FloatLiteral(_) => true,
            _ => false,
        }
    }

    /// Enforces that the given token is of a given kind.
    ///
    /// # Errors
    /// **This function will throw an error if the token is not of the given kind**.
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::lexer_tokenize;
    /// use calsc_lexer::toks::{Token, TokenKind};
    ///
    /// let tokens: Vec<Token> = lexer_tokenize("func", "test.cal".to_string()).unwrap();
    /// assert!(tokens[0].expects(TokenKind::Function).is_ok());
    ///
    pub fn expects(&self, kind: TokenKind) -> DiagPossible {
        if self.kind == kind {
            Ok(())
        } else {
            Err(build_expected_token_error(&kind, &self.kind, self).into())
        }
    }

    /// Enforces that the given token is of an integer literal
    /// and will return the literal's value if the token is of the given kind.
    ///
    /// # Errors
    /// **This function will throw an error if the token is not of the given kind**
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::lexer_tokenize;
    /// use calsc_lexer::toks::{Token, TokenKind};
    ///
    /// let tokens: Vec<Token> = lexer_tokenize("12", "test.cal".to_string()).unwrap();
    /// assert!(tokens[0].expects_int_lit().unwrap() == 12);
    ///
    pub fn expects_int_lit(&self) -> DiagResult<i128> {
        if let TokenKind::IntLiteral(v) = &self.kind {
            Ok(*v)
        } else {
            Err(build_expected_token_error(&"int literal".to_string(), &self.kind, self).into())
        }
    }

    /// Enforces that the given token is of an float literal
    /// and will return the literal's value if the token is of the given kind.
    ///
    /// # Errors
    /// **This function will throw an error if the token is not of the given kind**
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::lexer_tokenize;
    /// use calsc_lexer::toks::{Token, TokenKind};
    ///
    /// let tokens: Vec<Token> = lexer_tokenize("12.12", "test.cal".to_string()).unwrap();
    /// assert!(tokens[0].expects_float_lit().unwrap() == 12.12);
    ///
    pub fn expects_float_lit(&self) -> DiagResult<f64> {
        if let TokenKind::FloatLiteral(v) = &self.kind {
            Ok(*v)
        } else {
            Err(build_expected_token_error(&"float literal".to_string(), &self.kind, self).into())
        }
    }

    /// Enforces that the given token is of an string literal
    /// and will return the literal's value if the token is of the given kind.
    ///
    /// # Errors
    /// **This function will throw an error if the token is not of the given kind**
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::lexer_tokenize;
    /// use calsc_lexer::toks::{Token, TokenKind};
    ///
    /// let tokens: Vec<Token> = lexer_tokenize("\"test\"", "test.cal".to_string()).unwrap();
    /// assert!(tokens[0].expects_string_lit().unwrap() == String::from("test"));
    ///
    pub fn expects_string_lit(&self) -> DiagResult<String> {
        if let TokenKind::StringLiteral(v) = &self.kind {
            Ok(v.clone())
        } else {
            Err(build_expected_token_error(&"string literal".to_string(), &self.kind, self).into())
        }
    }

    /// Enforces that the given token is of an string literal
    /// and will return the literal's value if the token is of the given kind.
    ///
    /// # Errors
    /// **This function will throw an error if the token is not of the given kind**
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::lexer_tokenize;
    /// use calsc_lexer::toks::{Token, TokenKind};
    ///
    /// let tokens: Vec<Token> = lexer_tokenize("'\n'", "test.cal".to_string()).unwrap();
    /// assert!(tokens[0].expects_char_lit().unwrap() == '\n');
    ///
    pub fn expects_char_lit(&self) -> DiagResult<char> {
        if let TokenKind::CharLiteral(v) = &self.kind {
            Ok(*v)
        } else {
            Err(build_expected_token_error(&"char literal".to_string(), &self.kind, self).into())
        }
    }

    /// Enforces that the given token is of an keyword
    /// and will return the keyword's value if the token is of the given kind.
    ///
    /// # Errors
    /// **This function will throw an error if the token is not of the given kind**
    ///
    /// # Example
    /// ```
    /// use calsc_lexer::lexer_tokenize;
    /// use calsc_lexer::toks::{Token, TokenKind};
    ///
    /// let tokens: Vec<Token> = lexer_tokenize("test", "test.cal".to_string()).unwrap();
    /// assert!(tokens[0].expects_keyword().unwrap() == String::from("test"));
    ///
    pub fn expects_keyword(&self) -> DiagResult<String> {
        if let TokenKind::Keyword(v) = &self.kind {
            Ok(v.clone())
        } else {
            Err(build_expected_token_error(&"keyword".to_string(), &self.kind, self).into())
        }
    }
}

impl DiagnosticSource for Token {
    fn get_start_pos(&self) -> FilePosition {
        self.start.clone()
    }

    fn get_end_pos(&self) -> FilePosition {
        self.end.clone()
    }

    fn make_span(&self, kind: SpanKind, msg: Option<String>) -> Span {
        Span::new(kind, self.get_start_pos(), self.get_end_pos(), msg)
    }

    fn make_diagnostic_simple(
        &self,
        code: DiagnosticCode,
        message: String,
        primary_span_msg: Option<String>,
        spans: Vec<Span>,
        notes: Vec<String>,
        helps: Vec<String>,
    ) -> Diagnostic {
        Diagnostic::new(
            code,
            message,
            self.make_span(SpanKind::Primary, primary_span_msg),
            spans,
            notes,
            helps,
        )
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind // Uses a loosy check in order to only compare the kind
    }
}
impl Eq for Token {}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Function => "func",
            Self::True => "true",
            Self::False => "false",
            Self::If => "if",
            Self::Else => "else",
            Self::For => "for",
            Self::While => "while",
            Self::Loop => "loop",
            Self::ExternFunc => "externfunc",
            Self::Import => "import",
            Self::Var => "var",
            Self::Mut => "mut",
            Self::Struct => "struct",
            Self::Decl => "decl",
            Self::Return => "return",
            Self::Module => "module",
            Self::Public => "pub",
            Self::Protected => "prot",
            Self::Private => "priv",
            Self::Into => "into",
            Self::Match => "match",
            Self::SemiColon => ";",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::ParenOpen => "(",
            Self::ParenClose => ")",
            Self::BraceOpen => "{",
            Self::BraceClose => "}",
            Self::BracketOpen => "[",
            Self::BracketClose => "]",
            Self::At => "@",
            Self::Pound => "#",
            Self::Tilde => "~",
            Self::Question => "?",
            Self::Colon => ":",
            Self::Bang => "!",
            Self::AngelBracketOpen => "<",
            Self::AngelBracketClose => ">",
            Self::Minus => "-",
            Self::And => "&",
            Self::Or => "|",
            Self::Plus => "+",
            Self::Star => "*",
            Self::Slash => "/",
            Self::BackSlash => "\\",
            Self::Caret => "^",
            Self::Percent => "%",
            Self::Newline => "Newline",
            Self::Equal => "=",
            Self::Underscore => "_",
            Self::Keyword(str) => str,
            Self::StringLiteral(str) => str,
            Self::CharLiteral(char) => &format!("{}", char),
            Self::IntLiteral(val) => &format!("{}", val),
            Self::FloatLiteral(val) => &format!("{}", val),
            Self::Comment(_) => "comment",
            Self::Eof => "",
            Self::Unknown => "Unknown token",
        };

        write!(f, "{}", s)
    }
}

//! Lexer token related definitions.

use calsc_diagnostics::{
    Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};
use calsc_utils::pos::FilePosition;

/// A parsed token by the lexer.
/// Contains the kind of token as well as the data such as literal values.
/// Also contains a position of where the token starts and ends
pub struct Token {
    /// The kind of token. Holds the data such as literal values and the overall type of token
    pub kind: TokenKind,

    /// The start position of the token inside of the file
    pub start: FilePosition,

    /// The end position of the token inside of the file
    pub end: FilePosition,
}

/// Enum representing common lexer token kinds.
#[derive(PartialEq, Debug)]
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

    /// `else`
    Else,

    /// `externfunc`
    ExternFunc,

    /// `use`
    Use,

    /// `std`
    Std,

    /// `var`
    Var,

    /// `mut`
    Mut,

    /// `struct`
    Struct,

    /// `decl`
    Decl,

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

    /// // Comment
    LineComment,

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

    /// The end of file
    Eof,
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

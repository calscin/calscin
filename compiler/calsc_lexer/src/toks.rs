//! Lexer token related definitions.

use calsc_utils::pos::FilePosition;

/// A parsed token by the lexer.
/// Contains the kind of token as well as the data such as literal values.
/// Also contains a position of where the token starts and ends
pub struct Token {
    /// The kind of token. Holds the data such as literal values and the overall type of token
    pub kind: TokenKind,

    /// The position of the token inside of the file
    pub pos: FilePosition,
}

/// Enum representing common lexer token kinds.
pub enum TokenKind {
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
    /// let tok: Token = Token::new(TokenKind::Eof, pos);
    /// ```
    pub fn new(kind: TokenKind, pos: FilePosition) -> Self {
        Self { kind, pos }
    }
}

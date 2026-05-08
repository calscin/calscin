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

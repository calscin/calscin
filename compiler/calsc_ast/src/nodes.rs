//! Defines the tree of the AST. The AST is represented into a tree like structure where every "main" structure has children AST nodes themselves

/// The kind of AST tree node. Holds information about the node itself.
pub enum ASTNodeKind {
    /// An integer literal
    IntLiteral(i128),

    /// A float literal
    FloatLiteral(f64),

    /// A string literal
    StringLiteral(String),

    /// A char literal
    CharLiteral(char),
}

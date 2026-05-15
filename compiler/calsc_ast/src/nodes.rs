//! Defines the tree of the AST. The AST is represented into a tree like structure where every "main" structure has children AST nodes themselves

use std::collections::HashMap;

use calsc_diagnostics::{
    Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};
use calsc_utils::{
    cmp::CompareOperator, hash::HashedString, math::MathOperator, pos::FilePosition,
};

use crate::types::ASTType;

/// The kind of AST tree node. Holds information about the node itself.
#[derive(Debug, PartialEq)]
pub enum ASTNodeKind {
    /// An integer literal
    IntLiteral(i128),

    /// A float literal
    FloatLiteral(f64),

    /// A string literal
    StringLiteral(String),

    /// A char literal
    CharLiteral(char),

    /// A boolean literal
    BooleanLiteral(bool),

    /// The inverse condition representation (eg: !testS)
    InverseCondition(Box<ASTNode>),

    /// [start.end] -> incr
    Range {
        start: Box<ASTNode>,
        end: Box<ASTNode>,
        increment: Option<Box<ASTNode>>,
    },

    MathExpression {
        left_expr: Box<ASTNode>,
        right_expr: Box<ASTNode>,
        operator: MathOperator,
    },

    CompareExpression {
        left_expr: Box<ASTNode>,
        right_expr: Box<ASTNode>,
        operator: CompareOperator,
    },

    /// A variable declaration
    VariableDeclaration {
        mutable: bool,
        var_type: ASTType,
        name: HashedString,
        value: Option<Box<ASTNode>>,
    },

    StructuredInit {
        values: HashMap<HashedString, Box<ASTNode>>,
    },

    VariableReference(HashedString),

    /// `test = value`
    Assignment {
        variable: Box<ASTNode>,
        value: Box<ASTNode>,
    },

    FunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        body: Vec<Box<ASTNode>>,
    },

    ExternFunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        triple_dot_position: Option<usize>,
    },

    FunctionCall {
        name: HashedString,
        arguments: Vec<Box<ASTNode>>,
    },
}

/// Represents a real AST node. Holds information about the kind of AST node and it's start and end positions.
#[derive(Debug, PartialEq)]
pub struct ASTNode {
    pub kind: ASTNodeKind,
    pub start: FilePosition,
    pub end: FilePosition,
}

impl ASTNode {
    /// Creates a new AST node with the given kind, the given start and the given end.
    pub fn new(kind: ASTNodeKind, start: FilePosition, end: FilePosition) -> Self {
        Self { kind, start, end }
    }
}

impl DiagnosticSource for ASTNode {
    fn get_start_pos(&self) -> FilePosition {
        self.start.clone()
    }

    fn get_end_pos(&self) -> FilePosition {
        self.end.clone()
    }

    fn make_span(&self, kind: SpanKind, msg: Option<String>) -> Span {
        Span::new(kind, self.start.clone(), self.end.clone(), msg)
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

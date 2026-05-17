//! Defines the tree of the AST. The AST is represented into a tree like structure where every "main" structure has children AST nodes themselves

use std::collections::HashMap;

use calsc_diagnostics::{
    Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};
use calsc_utils::{
    alloc::arena::ArenaAllocatorReference, cmp::CompareOperator, hash::HashedString,
    math::MathOperator, pos::FilePosition,
};

use crate::{
    ifs::IfStatementBranch,
    imports::{ImportKind, ImportModule},
    types::ASTType,
};

/// The kind of AST tree node. Holds information about the node itself.
#[derive(Debug, PartialEq, Clone)]
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
    InverseCondition(ArenaAllocatorReference),

    PointerReference(ArenaAllocatorReference),
    PointerDereference(ArenaAllocatorReference),

    /// [start.end] -> incr
    Range {
        start: ArenaAllocatorReference,
        end: ArenaAllocatorReference,
        increment: Option<ArenaAllocatorReference>,
    },

    MathExpression {
        left_expr: ArenaAllocatorReference,
        right_expr: ArenaAllocatorReference,
        operator: MathOperator,
    },

    CompareExpression {
        left_expr: ArenaAllocatorReference,
        right_expr: ArenaAllocatorReference,
        operator: CompareOperator,
    },

    /// A variable declaration
    VariableDeclaration {
        mutable: bool,
        var_type: ASTType,
        name: HashedString,
        value: Option<ArenaAllocatorReference>,
    },

    StructuredInit {
        values: HashMap<HashedString, ArenaAllocatorReference>,
    },

    /// Refers to an element
    ElementReference(HashedString),

    /// `test = value`
    Assignment {
        variable: ArenaAllocatorReference,
        value: ArenaAllocatorReference,
    },

    FunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        body: Vec<ArenaAllocatorReference>,
    },

    ExternFunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        triple_dot_position: Option<usize>,
    },

    FunctionCall {
        name: HashedString,
        arguments: Vec<ArenaAllocatorReference>,
    },

    ForLoop {
        iterator_type: ASTType,
        iterator_name: HashedString,
        iterated: ArenaAllocatorReference,
        body: Vec<ArenaAllocatorReference>,
    },

    Loop {
        body: Vec<ArenaAllocatorReference>,
    },

    WhileLoop {
        condition: ArenaAllocatorReference,
        body: Vec<ArenaAllocatorReference>,
    },

    IfStatement {
        branches: Vec<IfStatementBranch>,
    },

    ImportStatement {
        /// The source / import module
        source: ImportModule,

        /// The path
        path: Vec<HashedString>,

        /// The kind of import
        kind: ImportKind,
    },

    StructDeclaration {
        name: HashedString,
        type_params: Vec<HashedString>,
        fields: Vec<(ASTType, HashedString)>,
    },

    StructDeclBlock {
        target: ASTType,
        functions: Vec<ArenaAllocatorReference>,
    },
}

/// Represents a real AST node. Holds information about the kind of AST node and it's start and end positions.
#[derive(Debug, PartialEq, Clone)]
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

impl ASTNodeKind {
    /// Does the node represent a body
    pub fn is_body(&self) -> bool {
        match self {
            Self::IfStatement { .. }
            | Self::ForLoop { .. }
            | Self::WhileLoop { .. }
            | Self::Loop { .. } => true,
            _ => false,
        }
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

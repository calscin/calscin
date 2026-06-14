//! Defines the tree of the AST. The AST is represented into a tree like structure where every "main" structure has children AST nodes themselves

use std::collections::HashMap;

use calsc_diagnostics::{
    Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};
use calsc_utils::{
    cmp::CompareOperator, hash::HashedString, math::MathOperator, pos::FilePosition,
};

use crate::{
    AST_CONTEXT,
    ifs::IfStatementBranch,
    imports::{ImportKind, ImportModule},
    path::ElementPath,
    refs::ASTArenaReference,
    types::ASTType,
};

/// The kind of AST tree node. Holds information about the node itself.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
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
    InverseCondition(ASTArenaReference),

    PointerReference(ASTArenaReference),
    PointerDereference(ASTArenaReference),

    /// [start.end] -> incr
    Range {
        start: ASTArenaReference,
        end: ASTArenaReference,
        increment: Option<ASTArenaReference>,
    },

    /// Represents any operation
    BinaryExpression {
        left_expr: ASTArenaReference,
        right_expr: ASTArenaReference,
        operator: BinaryOperator,
    },

    /// A variable declaration
    VariableDeclaration {
        mutable: bool,
        var_type: ASTType,
        name: HashedString,
        value: Option<ASTArenaReference>,
    },

    StructuredInit {
        values: HashMap<HashedString, ASTArenaReference>,
    },

    ArrayInit(Vec<ASTArenaReference>),

    /// Refers to an element
    ElementReference(HashedString),

    /// `test = value`
    Assignment {
        variable: ASTArenaReference,
        value: ASTArenaReference,
    },

    FunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        return_type: ASTType,
        body: Vec<ASTArenaReference>,
    },

    ExternFunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        return_type: ASTType,
        triple_dot_position: Option<usize>,
    },

    FunctionCall {
        name: ElementPath,
        arguments: Vec<ASTArenaReference>,
    },

    ReturnStatement {
        val: Option<ASTArenaReference>,
    },

    ForLoop {
        iterator_type: ASTType,
        iterator_name: HashedString,
        iterated: ASTArenaReference,
        body: Vec<ASTArenaReference>,
    },

    Loop {
        body: Vec<ASTArenaReference>,
    },

    WhileLoop {
        condition: ASTArenaReference,
        body: Vec<ASTArenaReference>,
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

    /// `test.abc`
    StructLRUsage {
        left_expr: ASTArenaReference,
        right_expr: ASTArenaReference,
    },

    IndexUsage {
        val: ASTArenaReference,
        index: ASTArenaReference,
    },

    StructDeclaration {
        name: HashedString,
        type_params: Vec<HashedString>,
        fields: Vec<(ASTType, HashedString)>,
    },

    StructDeclBlock {
        target: ASTType,
        functions: Vec<ASTArenaReference>,
    },

    Module {
        name: HashedString,
        body: Vec<ASTArenaReference>,
    },

    None,
}

/// Represents a real AST node. Holds information about the kind of AST node and it's start and end positions.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Clone)]
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

    /// Pushes the node into the arena allocator and consumes it
    pub fn push(self) -> ASTArenaReference {
        AST_CONTEXT.with(|f| f.borrow_mut().nodes.append(self))
    }

    pub fn get_top_level_name(&self) -> HashedString {
        match &self.kind {
            ASTNodeKind::FunctionDeclaration {
                name,
                arguments: _,
                return_type: _,
                body: _,
            } => name.clone(),

            ASTNodeKind::ExternFunctionDeclaration {
                name,
                arguments: _,
                return_type: _,
                triple_dot_position: _,
            } => name.clone(),
            ASTNodeKind::StructDeclaration {
                name,
                type_params: _,
                fields: _,
            } => name.clone(),

            _ => panic!("Cannot get level top level name on a non top level node"),
        }
    }

    pub fn is_additional_tree(&self) -> bool {
        match self.kind {
            ASTNodeKind::StructDeclBlock { .. } => true,
            _ => false,
        }
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

            Self::ReturnStatement { val } => val.is_none(),

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

/// Represents any binary operator
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Math(MathOperator),
    Compare(CompareOperator),
}

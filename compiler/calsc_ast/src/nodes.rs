//! Defines the tree of the AST. The AST is represented into a tree like structure where every "main" structure has children AST nodes themselves

use std::collections::HashMap;

use calsc_diagnostics::{
    Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};
use calsc_utils::{
    alloc::arena::ArenaHandle, cmp::CompareOperator, hash::HashedString, math::MathOperator,
    pos::FilePosition,
};

use crate::{
    ASTContext, ifs::IfStatementBranch, imports::ImportKind, path::ElementPath, types::ASTType,
    visibility::Visibility,
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
    InverseCondition(ArenaHandle),

    PointerReference(ArenaHandle, bool),
    PointerDereference(ArenaHandle),

    /// [start.end] -> incr
    Range {
        start: ArenaHandle,
        end: ArenaHandle,
        increment: Option<ArenaHandle>,
    },

    /// Represents any operation
    BinaryExpression {
        left_expr: ArenaHandle,
        right_expr: ArenaHandle,
        operator: BinaryOperator,
    },

    /// A variable declaration
    VariableDeclaration {
        mutable: bool,
        var_type: ASTType,
        name: HashedString,
        value: Option<ArenaHandle>,
    },

    StructuredInit {
        values: HashMap<HashedString, ArenaHandle>,
    },

    ArrayInit(Vec<ArenaHandle>),

    /// Refers to an element
    ElementReference(HashedString),

    /// `test = value`
    Assignment {
        variable: ArenaHandle,
        value: ArenaHandle,
    },

    FunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        return_type: ASTType,
        body: Vec<ArenaHandle>,
        visibility: Option<Visibility>,
        type_parameters: Vec<HashedString>,
    },

    ExternFunctionDeclaration {
        name: HashedString,
        arguments: Vec<(ASTType, HashedString)>,
        return_type: ASTType,
        triple_dot_position: Option<usize>,
        visibility: Option<Visibility>,
    },

    FunctionCall {
        name: ElementPath,
        arguments: Vec<ArenaHandle>,
    },

    ReturnStatement {
        val: Option<ArenaHandle>,
    },

    ForLoop {
        iterator_type: ASTType,
        iterator_name: HashedString,
        iterated: ArenaHandle,
        body: Vec<ArenaHandle>,
    },

    Loop {
        body: Vec<ArenaHandle>,
    },

    WhileLoop {
        condition: ArenaHandle,
        body: Vec<ArenaHandle>,
    },

    IfStatement {
        branches: Vec<IfStatementBranch>,
    },

    ImportStatement {
        /// The path
        path: ElementPath,

        /// The kind of import
        kind: ImportKind,
    },

    /// `test.abc`
    StructLRUsage {
        left_expr: ArenaHandle,
        right_expr: ArenaHandle,
    },

    IndexUsage {
        val: ArenaHandle,
        index: ArenaHandle,
    },

    StructDeclaration {
        name: HashedString,
        fields: Vec<(ASTType, HashedString)>,
        visibility: Option<Visibility>,
    },

    StructDeclBlock {
        target: ASTType,
        functions: Vec<ArenaHandle>,
    },

    Module {
        name: HashedString,
        is_bodied: bool,
        body: Vec<ArenaHandle>,
    },

    IntoCast {
        val: ArenaHandle,
        into: ASTType,
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
    pub fn push(self, ctx: &mut ASTContext) -> ArenaHandle {
        ctx.nodes.append(self)
    }

    pub fn get_top_level_name(&self) -> HashedString {
        match &self.kind {
            ASTNodeKind::FunctionDeclaration {
                name,
                arguments: _,
                return_type: _,
                body: _,
                visibility: _,
                type_parameters: _,
            } => name.clone(),

            ASTNodeKind::ExternFunctionDeclaration {
                name,
                arguments: _,
                return_type: _,
                triple_dot_position: _,
                visibility: _,
            } => name.clone(),
            ASTNodeKind::StructDeclaration {
                name,
                fields: _,
                visibility: _,
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

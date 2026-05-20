//! The node declarations for the HIR.

use std::collections::HashMap;

use calsc_typing::tree::Type;
use calsc_utils::{
    cmp::CompareOperator, hash::HashedString, math::MathOperator, pos::FilePosition,
};

use crate::refs::HIRArenaReference;

/// Represents the kind of the HIR node. Holds information related to the HIR node directly
pub enum HIRNodeKind {
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

    InverseCondition(HIRArenaReference),

    PointerReference(HIRArenaReference),
    PointerDereference(HIRArenaReference),

    Range {
        start: HIRArenaReference,
        end: HIRArenaReference,
        increment: Option<HIRArenaReference>,
    },

    MathExpression {
        left_expr: HIRArenaReference,
        right_expr: HIRArenaReference,
        operator: MathOperator,
    },

    CompareExpression {
        left_expr: HIRArenaReference,
        right_expr: HIRArenaReference,
        operator: CompareOperator,
    },

    VariableDeclaration {
        mutable: bool,
        var_type: Type,

        name: HashedString,

        /// The actual index representing the index inside of the local context
        variable_index: usize,
    },

    /// Reference for variable only
    VariableReference {
        name: HashedString,

        /// The actual index representing the index inside of the local context
        variable_index: usize,
    },

    StructuredInit {
        values: HashMap<HashedString, HIRArenaReference>,
    },

    ASsignment {
        variable: HIRArenaReference,
        value: HIRArenaReference,
    },

    ForLoop {
        iterator_type: Type,
        iterator_name: HashedString,

        /// The actual index representing the index inside of the local context
        iterator_variable_index: usize,

        iterated: HIRArenaReference,
        body: Vec<HIRArenaReference>,
    },

    Loop {
        body: Vec<HIRArenaReference>,
    },

    WhileLoop {
        condition: HIRArenaReference,
        body: Vec<HIRArenaReference>,
    },

    StructLRUsage {
        left_expr: HIRArenaReference,
        right_expr: HIRArenaReference,
    },
}

/// Represents a full HIR node. Holds the node kind and the start and end positions of it
pub struct HIRNode {
    pub kind: HIRNodeKind,
    pub start: FilePosition,
    pub end: FilePosition,
}

impl HIRNode {
    pub fn new(kind: HIRNodeKind, start: FilePosition, end: FilePosition) -> Self {
        Self { kind, start, end }
    }
}

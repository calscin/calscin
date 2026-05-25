//! The node declarations for the HIR.

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};
use calsc_typing::{FieldHavingType, tree::Type};
use calsc_utils::{
    cmp::CompareOperator, hash::HashedString, math::MathOperator, pos::FilePosition,
};

use crate::{
    HIR_CONTEXT,
    globalctx::key::GlobalContextKey,
    localctx::LocalContext,
    refs::HIRArenaReference,
    types::{make_bool_type, make_char_type, make_float_type, make_int_type, make_string_type},
};

/// Represents the kind of the HIR node. Holds information related to the HIR node directly
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
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

    FieldReference {
        name: HashedString,
    },

    FunctionReference {
        entry: GlobalContextKey,
    },

    StructuredInit {
        values: HashMap<HashedString, HIRArenaReference>,
    },

    Assignment {
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

    FunctionCall {
        func: GlobalContextKey,
        arguments: Vec<HIRArenaReference>,
    },

    FunctionDeclaration {
        key: GlobalContextKey,
        arguments: Vec<(Type, HashedString)>,
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
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct HIRNode {
    pub kind: HIRNodeKind,
    pub start: FilePosition,
    pub end: FilePosition,
}

impl HIRNode {
    pub fn new(kind: HIRNodeKind, start: FilePosition, end: FilePosition) -> Self {
        Self { kind, start, end }
    }

    pub fn push(self) -> HIRArenaReference {
        HIR_CONTEXT.with(|f| f.borrow_mut().nodes.append(self))
    }

    /// Gets the type of the [`HIRNode`] based on the node kind and the potentially given local context reference.
    ///
    /// # Errors & Panics
    /// This function will error if any type cannot be found or if the builder functions fails.
    ///
    /// This function will panic if references are wrong
    ///
    ///
    pub fn get_type(&self, local_ctx: Option<&LocalContext>) -> DiagResult<Option<Type>> {
        let ty = match self.kind.clone() {
            HIRNodeKind::IntLiteral(v) => Some(make_int_type(v < 0, 128, self)),
            HIRNodeKind::FloatLiteral(v) => Some(make_float_type(v < 0.0, 128, self)),
            HIRNodeKind::StringLiteral(_) => Some(make_string_type(self)),
            HIRNodeKind::CharLiteral(_) => Some(make_char_type(self)),
            HIRNodeKind::BooleanLiteral(_) => Some(make_bool_type(self)),
            HIRNodeKind::InverseCondition(_) => Some(make_bool_type(self)),

            HIRNodeKind::PointerDereference(_) => todo!(),
            HIRNodeKind::PointerReference(_) => todo!(),

            HIRNodeKind::MathExpression {
                left_expr,
                right_expr: _,
                operator,
            } => {
                if operator.assigns {
                    None
                } else {
                    left_expr.get_type(local_ctx)?
                }
            }

            HIRNodeKind::CompareExpression { .. } => Some(make_bool_type(self)),

            HIRNodeKind::VariableReference {
                name: _,
                variable_index,
            } => {
                if local_ctx.is_none() {
                    None
                } else {
                    Some(local_ctx.unwrap().variables[variable_index].ty.clone())
                }
            }

            HIRNodeKind::FunctionCall { func, arguments: _ } => {
                let ty = HIR_CONTEXT.with_borrow(|f| {
                    Ok(f.scope
                        .get_entry(func, self)?
                        .as_function(self)?
                        .return_type
                        .clone())
                });

                ty?
            }

            HIRNodeKind::StructLRUsage {
                left_expr,
                right_expr,
            } => {
                let ty = left_expr.get_type(local_ctx)?;

                if ty.is_none() {
                    None
                } else {
                    let ty = ty.unwrap();

                    match &right_expr.kind {
                        HIRNodeKind::FunctionCall { .. } => todo!(),
                        HIRNodeKind::FieldReference { name } => {
                            Some(ty.get_field_type(name.clone()))
                        } // The creation of FieldReference should check if the field is there

                        _ => panic!(),
                    }
                }
            }

            _ => None,
        };

        Ok(ty)
    }
}

impl DiagnosticSource for HIRNode {
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

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
    HIRContext,
    globalctx::key::GlobalContextKey,
    ifs::IfStatementBranch,
    refs::HIRArenaReference,
    types::{make_bool_type, make_char_type, make_float_type, make_int_type, make_string_type},
};

/// Represents the kind of the HIR node. Holds information related to the HIR node directly
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub enum HIRNodeKind {
    /// An integer literal
    IntLiteral(i128, usize, bool),

    /// A float literal
    FloatLiteral(f64, usize, bool),

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

        value: Option<HIRArenaReference>,

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
        val: HIRArenaReference,
        field_ind: usize,
        name: HashedString,
    },

    IndexUsage {
        val: HIRArenaReference,
        index: HIRArenaReference,
        output_type: Type,
    },

    FunctionReference {
        entry: GlobalContextKey,
    },

    StructuredInit {
        values: HashMap<HashedString, HIRArenaReference>,
    },

    TypedStructuredInit {
        ty: Type,
        values: HashMap<HashedString, HIRArenaReference>,
    },

    ArrayInit {
        vals: Vec<HIRArenaReference>,
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
        return_type: Type,
        append_terminator: bool,
    },

    ReturnStatement {
        val: Option<HIRArenaReference>,
    },

    Loop {
        body: Vec<HIRArenaReference>,
    },

    WhileLoop {
        condition: HIRArenaReference,
        body: Vec<HIRArenaReference>,
    },

    IfStatement {
        branches: Vec<IfStatementBranch>,
    },

    CastNode {
        original: HIRArenaReference,
        into: Type,
    },
}

/// Represents a full HIR node. Holds the node kind and the start and end positions of it
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct HIRNode {
    pub kind: HIRNodeKind,
    pub start: FilePosition,
    pub end: FilePosition,
    pub stronger_type: Option<Type>,
}

impl HIRNode {
    pub fn new(kind: HIRNodeKind, start: FilePosition, end: FilePosition) -> Self {
        Self {
            kind,
            start,
            end,
            stronger_type: None,
        }
    }

    pub fn push(self, ctx: &mut HIRContext) -> HIRArenaReference {
        ctx.nodes.append(self)
    }

    /// Gets the type of the [`HIRNode`] based on the node kind and the potentially given local context reference.
    ///
    /// # Errors & Panics
    /// This function will error if any type cannot be found or if the builder functions fails.
    ///
    /// This function will panic if references are wrong
    ///
    ///
    pub fn get_type(
        &self,
        local_func_key: Option<GlobalContextKey>,
        ctx: &HIRContext,
    ) -> DiagResult<Type> {
        if self.stronger_type.is_some() {
            return Ok(self.stronger_type.clone().unwrap());
        }

        let ty = match self.kind.clone() {
            HIRNodeKind::IntLiteral(_, size, signed) => make_int_type(signed, size, self, ctx),
            HIRNodeKind::FloatLiteral(_, size, signed) => make_float_type(signed, size, self, ctx),
            HIRNodeKind::StringLiteral(_) => make_string_type(self, ctx),
            HIRNodeKind::CharLiteral(_) => make_char_type(self, ctx),
            HIRNodeKind::BooleanLiteral(_) => make_bool_type(self, ctx),
            HIRNodeKind::InverseCondition(_) => make_bool_type(self, ctx),

            HIRNodeKind::Range {
                start,
                end: _,
                increment: _,
            } => start.get_type(local_func_key, ctx)?,

            HIRNodeKind::PointerReference(val) => Type::Reference {
                mutable: true, // Mutable by default, will change
                inner: Box::new(val.get_type(local_func_key, ctx)?),
            },

            HIRNodeKind::PointerDereference(val) => {
                val.get_type(local_func_key, ctx)?.get_inner() // Assumes the container of a pointer reference is a pointer.
            }

            HIRNodeKind::MathExpression {
                left_expr,
                right_expr: _,
                operator,
            } => {
                if operator.assigns {
                    Type::Void
                } else {
                    left_expr.get_type(local_func_key, ctx)?
                }
            }

            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name,
            } => {
                let ty = val.get_type(local_func_key, ctx)?;

                if ty.has_field(name.clone()) {
                    ty.get_field_type(name)
                } else {
                    Type::Void
                }
            }

            HIRNodeKind::CompareExpression { .. } => make_bool_type(self, ctx),

            HIRNodeKind::VariableReference {
                name: _,
                variable_index,
            } => {
                if local_func_key.is_none() {
                    Type::Void
                } else {
                    ctx.scope
                        .get_entry(local_func_key.unwrap(), self)?
                        .as_function(self)?
                        .local_context
                        .as_ref()
                        .unwrap()
                        .variables[variable_index]
                        .ty
                        .clone()
                }
            }

            HIRNodeKind::FunctionCall { func, arguments: _ } => ctx
                .scope
                .get_entry(func, self)?
                .as_function(self)?
                .return_type
                .clone(),

            HIRNodeKind::IndexUsage {
                val: _,
                index: _,
                output_type,
            } => output_type,

            HIRNodeKind::ArrayInit { vals } => Type::Array {
                size: Some(vals.len()),
                inner: Box::new(vals[0].get_type(local_func_key, ctx)?),
            },

            _ => Type::Void,
        };

        Ok(ty)
    }

    pub fn is_numerical_lit(&self) -> bool {
        match self.kind {
            HIRNodeKind::IntLiteral(_, _, _) => true,
            HIRNodeKind::FloatLiteral(_, _, _) => true,
            _ => false,
        }
    }

    /// Does the node represent a pointer referencable
    pub fn represents_pointer_referencable(&self) -> bool {
        match &self.kind {
            HIRNodeKind::VariableReference { .. } => true,
            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name: _,
            } => val.represents_pointer_referencable(),

            HIRNodeKind::IndexUsage {
                val,
                index: _,
                output_type: _,
            } => val.represents_pointer_referencable(),

            _ => false,
        }
    }

    /// Does the node represent a mutable variable-like
    pub fn represents_mutable_variable(&self) -> bool {
        match &self.kind {
            HIRNodeKind::VariableReference { .. } => true,
            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name: _,
            } => val.represents_mutable_variable(),
            HIRNodeKind::PointerDereference(inner) => inner.represents_mutable_variable(),
            HIRNodeKind::IndexUsage {
                val,
                index: _,
                output_type: _,
            } => val.represents_mutable_variable(),

            _ => false,
        }
    }

    /// Gets the root variable index of the node
    pub fn get_root_variable_reference_index(&self) -> usize {
        match &self.kind {
            HIRNodeKind::VariableReference {
                name: _,
                variable_index,
            } => *variable_index,

            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name: _,
            } => val.get_root_variable_reference_index(),

            HIRNodeKind::PointerDereference(inner) => inner.get_root_variable_reference_index(),

            HIRNodeKind::IndexUsage {
                val,
                index: _,
                output_type: _,
            } => val.get_root_variable_reference_index(),

            #[cfg(feature = "debug")]
            kind => panic!("Unexpected variable reference kind {:#?}", kind),

            #[cfg(not(feature = "debug"))]
            _ => panic!("Unexpected variable reference kind"),
        }
    }

    pub fn is_weakly_typed(&self) -> bool {
        match &self.kind {
            HIRNodeKind::IntLiteral(_, _, _) => true,
            HIRNodeKind::FloatLiteral(_, _, _) => true,
            HIRNodeKind::MathExpression {
                left_expr,
                right_expr,
                operator: _,
            } => left_expr.is_weakly_typed() && right_expr.is_weakly_typed(),

            HIRNodeKind::Range {
                start,
                end,
                increment,
            } => {
                start.is_weakly_typed()
                    && end.is_weakly_typed()
                    && (increment.is_none() || increment.as_ref().unwrap().is_weakly_typed())
            }

            HIRNodeKind::ArrayInit { vals } => {
                for val in vals {
                    if !val.is_weakly_typed() {
                        return false;
                    }
                }

                true
            }

            _ => false,
        }
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

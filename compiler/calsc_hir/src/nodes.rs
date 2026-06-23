//! The node declarations for the HIR.

use std::collections::HashMap;

use calsc_diagnostics::{
    DiagResult, Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};

use calsc_typing_v2::types::{MutationState, TypeKind};
use calsc_utils::{
    alloc::arena::ArenaHandle, cmp::CompareOperator, hash::HashedString, math::MathOperator,
    pos::FilePosition,
};

use crate::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    ifs::IfStatementBranch,
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

    InverseCondition(ArenaHandle),

    PointerReference(ArenaHandle, MutationState),
    PointerDereference(ArenaHandle),

    Range {
        start: ArenaHandle,
        end: ArenaHandle,
        increment: Option<ArenaHandle>,
    },

    MathExpression {
        left_expr: ArenaHandle,
        right_expr: ArenaHandle,
        operator: MathOperator,
    },

    CompareExpression {
        left_expr: ArenaHandle,
        right_expr: ArenaHandle,
        operator: CompareOperator,
    },

    VariableDeclaration {
        mutable: bool,
        var_type: TypeKind,

        value: Option<ArenaHandle>,

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
        val: ArenaHandle,
        field_ind: usize,
        name: HashedString,
    },

    IndexUsage {
        val: ArenaHandle,
        index: ArenaHandle,
        output_type: TypeKind,
    },

    FunctionReference {
        entry: GlobalContextKey,
    },

    StructuredInit {
        values: HashMap<HashedString, ArenaHandle>,
    },

    TypedStructuredInit {
        ty: TypeKind,
        values: HashMap<HashedString, ArenaHandle>,
    },

    ArrayInit {
        vals: Vec<ArenaHandle>,
    },

    Assignment {
        variable: ArenaHandle,
        value: ArenaHandle,
    },

    ForLoop {
        iterator_type: TypeKind,
        iterator_name: HashedString,

        /// The actual index representing the index inside of the local context
        iterator_variable_index: usize,

        iterated: ArenaHandle,
        body: Vec<ArenaHandle>,
    },

    FunctionCall {
        func: GlobalContextKey,
        arguments: Vec<ArenaHandle>,
    },

    FunctionDeclaration {
        key: GlobalContextKey,
        arguments: Vec<(TypeKind, HashedString)>,
        body: Vec<ArenaHandle>,
        return_type: TypeKind,
        append_terminator: bool,
    },

    ReturnStatement {
        val: Option<ArenaHandle>,
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

    CastNode {
        original: ArenaHandle,
        into: TypeKind,

        /// Represents whenever the cast was done explicitly by the user (using into)
        explicit_cast: bool,
    },
}

/// Represents a full HIR node. Holds the node kind and the start and end positions of it
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct HIRNode {
    pub kind: HIRNodeKind,
    pub start: FilePosition,
    pub end: FilePosition,
    pub stronger_type: Option<TypeKind>,
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

    pub fn push(self, ctx: &mut HIRContext) -> ArenaHandle {
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
        ctx: &mut HIRContext,
        file_ctx: Option<&HIRFileContext>,
    ) -> DiagResult<TypeKind> {
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
            } => ctx
                .nodes
                .get(&start)
                .get_type(local_func_key, ctx, file_ctx)?,

            HIRNodeKind::PointerReference(val, mutable) => {
                let ty = ctx
                    .nodes
                    .get(&val)
                    .get_type(local_func_key, ctx, file_ctx)?;
                let ty = ctx.type_ctx.type_kind_arena.append(ty);

                TypeKind::Reference(mutable, ty)
            }

            HIRNodeKind::PointerDereference(val) => ctx
                .nodes
                .get(&val)
                .get_type(local_func_key, ctx, file_ctx)?
                .get_inner(&ctx.type_ctx)
                .clone(),

            HIRNodeKind::MathExpression {
                left_expr,
                right_expr: _,
                operator,
            } => {
                if operator.assigns {
                    TypeKind::Void
                } else {
                    ctx.nodes
                        .get(&left_expr)
                        .get_type(local_func_key, ctx, file_ctx)?
                }
            }

            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name,
            } => {
                let ty = ctx
                    .nodes
                    .get(&val)
                    .get_type(local_func_key, ctx, file_ctx)?;

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
                    let entry;

                    if file_ctx.is_some() {
                        entry = ctx.scope.get_entry(
                            local_func_key.unwrap(),
                            &file_ctx.unwrap().current_module,
                            self,
                        )?
                    } else {
                        entry = ctx
                            .scope
                            .get_entry_no_visibility(local_func_key.unwrap(), self)?
                    }

                    entry
                        .as_function(self)?
                        .local_context
                        .as_ref()
                        .unwrap()
                        .variables[variable_index]
                        .ty
                        .clone()
                }
            }

            HIRNodeKind::FunctionCall { func, arguments: _ } => {
                let entry;

                if file_ctx.is_some() {
                    entry = ctx
                        .scope
                        .get_entry(func, &file_ctx.unwrap().current_module, self)?
                } else {
                    entry = ctx.scope.get_entry_no_visibility(func, self)?
                }

                entry.as_function(self)?.return_type.clone()
            }

            HIRNodeKind::IndexUsage {
                val: _,
                index: _,
                output_type,
            } => output_type,

            HIRNodeKind::ArrayInit { vals } => {
                let ty = ctx
                    .nodes
                    .get(&vals[0])
                    .get_type(local_func_key, ctx, file_ctx)?;

                let ty = ctx.type_ctx.type_kind_arena.append(ty);

                TypeKind::Array(vals.len(), ty)
            }

            HIRNodeKind::CastNode {
                original: _,
                into,
                explicit_cast: _,
            } => into.clone(),

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
    pub fn represents_pointer_referencable(&self, ctx: &HIRContext) -> bool {
        match &self.kind {
            HIRNodeKind::VariableReference { .. } => true,
            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name: _,
            } => ctx.nodes.get(val).represents_pointer_referencable(ctx),

            HIRNodeKind::IndexUsage {
                val,
                index: _,
                output_type: _,
            } => ctx.nodes.get(val).represents_pointer_referencable(ctx),

            _ => false,
        }
    }

    /// Does the node represent a mutable variable-like
    pub fn represents_mutable_variable<S: DiagnosticSource>(
        &self,
        ctx: &mut HIRContext,
        local_func_key: Option<GlobalContextKey>,
        source: &S,
    ) -> DiagResult<bool> {
        match &self.kind {
            HIRNodeKind::VariableReference {
                variable_index,
                name: _,
            } => {
                if local_func_key.is_none() {
                    return Ok(false); // Handles static variables potentially
                }

                let local_ctx = ctx
                    .scope
                    .get_entry_no_visibility(local_func_key.unwrap(), source)?
                    .as_function(source)?
                    .local_context
                    .as_ref()
                    .unwrap();

                Ok(local_ctx.variables[*variable_index].mutable)
            }

            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name: _,
            } => ctx
                .nodes
                .get(val)
                .represents_mutable_variable(ctx, local_func_key, source),

            HIRNodeKind::PointerDereference(inner) => {
                let ty = ctx.nodes.get(inner).get_type(local_func_key, ctx, None)?;

                Ok(ty.is_type_mutable_compatible())
            }

            HIRNodeKind::PointerReference(inner, mutable) => Ok(mutable.0
                && ctx.nodes.get(inner).represents_mutable_variable(
                    ctx,
                    local_func_key,
                    source,
                )?),

            HIRNodeKind::IndexUsage {
                val,
                index: _,
                output_type: _,
            } => ctx
                .nodes
                .get(val)
                .represents_mutable_variable(ctx, local_func_key, source),

            _ => Ok(false),
        }
    }

    /// Gets the root variable index of the node
    pub fn get_root_variable_reference_index(&self, ctx: &HIRContext) -> usize {
        match &self.kind {
            HIRNodeKind::VariableReference {
                name: _,
                variable_index,
            } => *variable_index,

            HIRNodeKind::FieldReference {
                val,
                field_ind: _,
                name: _,
            } => ctx.nodes.get(val).get_root_variable_reference_index(ctx),

            HIRNodeKind::PointerDereference(inner) => {
                ctx.nodes.get(inner).get_root_variable_reference_index(ctx)
            }

            HIRNodeKind::IndexUsage {
                val,
                index: _,
                output_type: _,
            } => ctx.nodes.get(val).get_root_variable_reference_index(ctx),

            #[cfg(feature = "debug")]
            kind => panic!("Unexpected variable reference kind {:#?}", kind),

            #[cfg(not(feature = "debug"))]
            _ => panic!("Unexpected variable reference kind"),
        }
    }

    pub fn is_weakly_typed(&self, ctx: &HIRContext) -> bool {
        match &self.kind {
            HIRNodeKind::IntLiteral(_, _, _) => true,
            HIRNodeKind::FloatLiteral(_, _, _) => true,
            HIRNodeKind::MathExpression {
                left_expr,
                right_expr,
                operator: _,
            } => {
                ctx.nodes.get(left_expr).is_weakly_typed(ctx)
                    && ctx.nodes.get(right_expr).is_weakly_typed(ctx)
            }

            HIRNodeKind::Range {
                start,
                end,
                increment,
            } => {
                ctx.nodes.get(start).is_weakly_typed(ctx)
                    && ctx.nodes.get(end).is_weakly_typed(ctx)
                    && (increment.is_none()
                        || ctx
                            .nodes
                            .get(increment.as_ref().unwrap())
                            .is_weakly_typed(ctx))
            }

            HIRNodeKind::ArrayInit { vals } => {
                for val in vals {
                    if !ctx.nodes.get(val).is_weakly_typed(ctx) {
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

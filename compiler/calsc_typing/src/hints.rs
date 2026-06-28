//! Type hints allow for type determination with a simple system.
//!
//! This system is experimental and may be used for later purposes

use calsc_diagnostics::{
    DiagResult, DiagnosticSource, diags::errors::build_type_hint_coherce_not_transmutable,
};
use calsc_utils::display_with_to_string;

use crate::{ctx::TypeCtx, into::TypeTransmutation, types::TypeKind};

#[derive(Clone)]
pub enum TypeHint {
    /// A strong type hint. Represents a type hint that cannot be overriden.
    /// There can only be a single type hint per query.
    Strong(TypeKind),

    /// A weak type hint. Represents a type hint that can be overriden.
    /// There can be multiple weak hints per query.
    Weak(TypeKind),
}

#[derive(Clone)]
pub struct TypeHintContainer {
    pub strong_hints: Vec<TypeHint>,
    pub weak_hints: Vec<TypeHint>,
}

impl TypeHintContainer {
    /// Creates a new [`TypeHintContainer`]
    pub fn new() -> Self {
        Self {
            strong_hints: vec![],
            weak_hints: vec![],
        }
    }

    /// Appends a new [`TypeHint`] inside of the [`TypeHintContainer`]
    pub fn append(&mut self, hint: TypeHint) {
        if hint.is_strong() {
            self.strong_hints.push(hint);
        } else {
            self.weak_hints.push(hint);
        }
    }

    /// Determines the type held by the hints by applying the type hint coercition rules.
    /// These rules are:
    /// - There is one "master" type, selected by being either the first strong hint or the first weak hint.
    /// - Every other strong hint must be directly transmutable into the "master" type
    /// - Every other weak hint must be directly weakly transmutable into the "master" type.
    pub fn determine_type<S: DiagnosticSource>(
        &self,
        ctx: &TypeCtx,
        source: &S,
    ) -> DiagResult<TypeKind> {
        let master = if !self.strong_hints.is_empty() {
            &self.strong_hints[0]
        } else {
            &self.weak_hints[0]
        };

        for ind in 1..self.strong_hints.len() {
            let entry = &self.strong_hints[ind];

            if !entry.get_type().can_transmute(master.get_type(), ctx) {
                return Err(build_type_hint_coherce_not_transmutable(
                    &display_with_to_string(master.get_type(), ctx),
                    &display_with_to_string(entry.get_type(), ctx),
                    source,
                )
                .into());
            }
        }

        for entry in &self.weak_hints {
            if !entry
                .get_type()
                .can_transmute_weakly(master.get_type(), ctx)
            {
                return Err(build_type_hint_coherce_not_transmutable(
                    &display_with_to_string(master.get_type(), ctx),
                    &display_with_to_string(entry.get_type(), ctx),
                    source,
                )
                .into());
            }
        }

        Ok(master.get_type().clone())
    }
}

impl TypeHint {
    /// Checks if the type hint is a strong hint.
    pub fn is_strong(&self) -> bool {
        matches!(self, TypeHint::Strong(_))
    }

    /// Gets the type hint's type.
    pub fn get_type(&self) -> &TypeKind {
        match self {
            Self::Strong(ty) => ty,
            Self::Weak(ty) => ty,
        }
    }
}

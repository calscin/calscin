//! The definitions for the diagnostic container. It's main purpose is to contain diagnostics statically

use std::cell::RefCell;

use calsc_utils::pos::FilePosition;

use crate::{
    Diagnostic, DiagnosticCode, Level,
    span::{Span, SpanKind},
};

thread_local! {
    static DIAGNOSTIC_CONTAINER: RefCell<Vec<Diagnostic>> = RefCell::new(vec![]);
}

/// Represents a source of diagnostics. A source of diagnostics should be able to do the following:
/// - Create a span
/// - Create a diagnostic
/// - Have a start position
/// - Have an ending position
///
/// This trait ensures that the given source follows these conditions
pub trait DiagnosticSource {
    /// Makes a span based on the source's position with the given kind and the given message
    fn make_span(&self, kind: SpanKind, msg: Option<String>) -> Span;

    /// Makes a simple diagnostic with the primary span at the source's position with the given code, message,
    /// primary span message, additional spans, notes and help messages
    fn make_diagnostic_simple(
        &self,
        code: DiagnosticCode,
        message: String,
        primary_span_msg: Option<String>,
        spans: Vec<Span>,
        notes: Vec<String>,
        helps: Vec<String>,
    ) -> Diagnostic;

    /// Gets the source's start position
    fn get_start_pos(&self) -> FilePosition;

    /// Get's the source ending position
    fn get_end_pos(&self) -> FilePosition;
}

pub(crate) fn push_diagnostic(diagnostic: Diagnostic) {
    DIAGNOSTIC_CONTAINER.with_borrow_mut(|f| f.push(diagnostic))
}

pub fn has_diagnostics() -> bool {
    DIAGNOSTIC_CONTAINER.with_borrow(|f| !f.is_empty())
}

pub fn has_errors() -> bool {
    let mut has_errors = false;

    DIAGNOSTIC_CONTAINER.with_borrow(|f| {
        for d in f {
            if d.code.level == Level::Error {
                has_errors = true;
                break;
            }
        }
    });

    has_errors
}

pub fn dump_diagnostics() {
    DIAGNOSTIC_CONTAINER.with_borrow(|f| {
        for d in f {
            println!("{}", d);
        }
    })
}

pub fn clear_diagnostics() {
    DIAGNOSTIC_CONTAINER.with_borrow_mut(|f| f.clear());
}

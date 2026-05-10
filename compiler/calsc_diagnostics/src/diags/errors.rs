//! The error declarations

use std::fmt::Display;

use crate::{Diagnostic, DiagnosticCode, DiagnosticSource, Level, declare_diagnostic};

// Is triggered whenever the Lexer cannot parse something (eg: cannot parse literals).
declare_diagnostic!(CANNOT_PARSE, 1);
declare_diagnostic!(UNEXPECTED_TOKEN, 2);
declare_diagnostic!(EXPECTED, 3);

/// Builds a `CANNOT_PARSE` error (E1) based on the given source and given element.
pub fn build_cannot_parse_error<P: Display, S: DiagnosticSource>(p: &P, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, CANNOT_PARSE),
        format!("cannot parse {}", p),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_unexpected_error<E: Display, S: DiagnosticSource>(elem: &E, source: &S) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, UNEXPECTED_TOKEN),
        format!("unexpected {}", elem),
        None,
        vec![],
        vec![],
        vec![],
    )
}

pub fn build_expected_error<E: Display, G: Display, S: DiagnosticSource>(
    expected: &E,
    got: &G,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(Level::Error, UNEXPECTED_TOKEN),
        format!("expected {} but got {}", expected, got),
        None,
        vec![],
        vec![],
        vec![],
    )
}

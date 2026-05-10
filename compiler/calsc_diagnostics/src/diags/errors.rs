//! The error declarations

use std::fmt::Display;

use crate::{Diagnostic, DiagnosticCode, DiagnosticSource, Level, declare_diagnostic};

// Is triggered whenever the Lexer cannot parse something (eg: cannot parse literals).
declare_diagnostic!(CANNOT_PARSE, 1);

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

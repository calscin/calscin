//! The diagnostic system of Calscin.
//! Allows for clean errors, warning and information messages at all stages of the language.
//! The main error system are diagnostics

#[cfg(feature = "backtraces")]
use std::backtrace::Backtrace;

use crate::span::Span;

pub mod fmt;
pub mod span;

/// The level of diagnostics. Represents the type of diagnostic (eg: error, warning or information).
#[derive(Clone)]
pub enum Level {
    Error,
    Warning,
    Info,
}

/// Represents a code to represent diagnostics from each other. Also called a warning / error code.
/// Contains the diagnostic level and a code representing the diagnostic.
///
/// # Example
/// ```
/// use calsc_diagnostics::{Level, DiagnosticCode};
///
/// // Represents the diagnostic "E123"
/// let code: DiagnosticCode = DiagnosticCode::new(Level::Error, 123);
/// ```
#[derive(Clone)]
pub struct DiagnosticCode {
    pub level: Level,
    pub code: usize,
}

/// Represents a diagnostic inside of the diagnostic system. Can be directly used with a formatter to get the display version of the Diagnostic.
/// Contains the diagnostic code, the message of the diagnostic, the spans, info messages and help messages.
/// Also potentially holds a backtrace if the `backtrace` feature is enabled
#[derive(Clone)]
pub struct Diagnostic {
    /// The warning / error code of the diagnostic
    pub code: DiagnosticCode,

    /// The message of the diagnostic
    pub message: String,

    /// The primary span of the Diagnostic
    pub primary_span: Span,

    /// The secondary spans of the Diagnostic
    pub spans: Vec<Span>,

    pub notes: Vec<String>,
    pub helps: Vec<String>,

    #[cfg(feature = "backtraces")]
    pub backtrace: String,
}

impl DiagnosticCode {
    /// Creates a new diagnostic code with the given diagnostic level and the given warn / error code.
    ///
    /// # Example
    /// ```
    /// use calsc_diagnostics::{Level, DiagnosticCode};
    ///
    /// // Represents the diagnostic "E123"
    /// let code: DiagnosticCode = DiagnosticCode::new(Level::Error, 123);
    /// ```
    pub fn new(level: Level, code: usize) -> Self {
        Self { level, code }
    }
}

impl Diagnostic {
    /// Creates a new diagnostic based on the given code, message, primary span, additional spans, notes and helping notes.
    ///
    /// # Example
    /// ```
    /// use calsc_diagnostics::{Level, DiagnosticCode, Diagnostic};
    /// use calsc_diagnostics::span::{Span, SpanKind};
    /// use std::path::PathBuf;
    /// use calsc_utils::pos::FilePosition;
    ///
    /// // Represents the diagnostic "E123"
    /// let code: DiagnosticCode = DiagnosticCode::new(Level::Error, 123);
    ///
    /// let start: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let end: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 35);
    ///
    /// let span: Span = Span::new(SpanKind::Primary, start, end, Some("my label".to_string()));
    ///
    /// let diagnostic: Diagnostic = Diagnostic::new(code, "My message".to_string(), span, vec![], vec![], vec![]);
    /// ```
    pub fn new(
        code: DiagnosticCode,
        message: String,
        primary_span: Span,
        spans: Vec<Span>,
        notes: Vec<String>,
        helps: Vec<String>,
    ) -> Self {
        let d;

        #[cfg(not(feature = "backtraces"))]
        {
            d = Self {
                code,
                message,
                primary_span,
                spans,
                notes,
                helps,
            }
        }

        #[cfg(feature = "backtraces")]
        {
            d = Self {
                code,
                message,
                primary_span,
                spans,
                notes,
                helps,
                backtrace: format!("{}", Backtrace::capture()),
            };
        }

        d
    }
}

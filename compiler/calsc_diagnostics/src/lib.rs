//! The diagnostic system of Calscin.
//! Allows for clean errors, warning and information messages at all stages of the language.
//! The main error system are diagnostics

pub mod fmt;
pub mod span;

/// The level of diagnostics. Represents the type of diagnostic (eg: error, warning or information).
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
pub struct DiagnosticCode {
    pub level: Level,
    pub code: usize,
}

pub struct Diagnostic {
    pub code: DiagnosticCode,
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

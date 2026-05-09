//! The span system of the diagnostics. Allows diagnostics to have multiple positions that point to multiple informations.
//! Example: an error that points to an introduction point and the failure point.

use calsc_utils::pos::FilePosition;

/// Struct representing a span.
/// Holds a position, a label and a span kind.
#[derive(Clone)]
pub struct Span {
    pub start: FilePosition,
    pub end: FilePosition,

    pub label: Option<String>,
    pub kind: SpanKind,
}

/// The kind of `Span`. Can be either a primary span or a secondary span.
#[derive(Clone)]
pub enum SpanKind {
    Primary,
    Secondary,
}

impl Span {
    /// Creates a new span.
    ///
    /// # Example
    ///
    /// ```
    /// use calsc_diagnostics::span::{Span, SpanKind};
    /// use std::path::PathBuf;
    /// use calsc_utils::pos::FilePosition;
    ///
    /// let start: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 28);
    /// let end: FilePosition = FilePosition::new(PathBuf::from("./test"), 1, 35);
    ///
    /// let span: Span = Span::new(SpanKind::Primary, start, end, Some("my label".to_string()));
    /// ```
    pub fn new(
        kind: SpanKind,
        start: FilePosition,
        end: FilePosition,
        label: Option<String>,
    ) -> Self {
        Self {
            start,
            end,
            label,
            kind,
        }
    }
}

impl SpanKind {
    /// Get the display char for the span kind.
    pub fn get_char(&self) -> char {
        match self {
            Self::Primary => '^',
            Self::Secondary => '-',
        }
    }
}

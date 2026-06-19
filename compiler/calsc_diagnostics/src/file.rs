use std::{fs, path::PathBuf};

use calsc_utils::pos::FilePosition;

use crate::{
    Diagnostic, DiagnosticCode, DiagnosticSource,
    span::{Span, SpanKind},
};

/// Represents a diagnostic origin that is a whole file
pub struct FileDiagnosticPos {
    pub file: PathBuf,
    pub start: FilePosition,
    pub end: FilePosition,
}

impl FileDiagnosticPos {
    pub fn new(file: PathBuf) -> Self {
        let str = fs::read_to_string(&file).unwrap();
        let lines: Vec<_> = str.lines().collect();

        let last_line_len = lines[lines.len() - 1].len();

        let end = FilePosition::new(file.clone(), lines.len(), last_line_len);

        Self {
            file: file.clone(),
            start: FilePosition::new(file.clone(), 0, 0),
            end,
        }
    }
}

impl DiagnosticSource for FileDiagnosticPos {
    fn get_start_pos(&self) -> FilePosition {
        self.start.clone()
    }

    fn get_end_pos(&self) -> FilePosition {
        self.end.clone()
    }

    fn make_span(&self, kind: crate::span::SpanKind, msg: Option<String>) -> Span {
        Span {
            start: self.start.clone(),
            end: self.end.clone(),
            label: msg,
            kind,
        }
    }

    fn make_diagnostic_simple(
        &self,
        code: DiagnosticCode,
        message: String,
        primary_span_msg: Option<String>,
        spans: Vec<Span>,
        notes: Vec<String>,
        helps: Vec<String>,
    ) -> crate::Diagnostic {
        let primary_span = self.make_span(SpanKind::Primary, primary_span_msg);

        Diagnostic::new(code, message, primary_span, spans, notes, helps)
    }
}

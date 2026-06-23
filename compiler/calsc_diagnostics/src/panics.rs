use crate::DiagnosticSource;

pub struct PanicDiagnosticSource();

impl DiagnosticSource for PanicDiagnosticSource {
    fn get_end_pos(&self) -> calsc_utils::pos::FilePosition {
        panic!()
    }

    fn get_start_pos(&self) -> calsc_utils::pos::FilePosition {
        panic!()
    }

    fn make_span(&self, _kind: crate::span::SpanKind, _msg: Option<String>) -> crate::span::Span {
        panic!()
    }

    fn make_diagnostic_simple(
        &self,
        _code: crate::DiagnosticCode,
        _message: String,
        _primary_span_msg: Option<String>,
        _spans: Vec<crate::span::Span>,
        _notes: Vec<String>,
        _helps: Vec<String>,
    ) -> crate::Diagnostic {
        panic!()
    }
}

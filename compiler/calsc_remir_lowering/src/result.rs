use calsc_diagnostics::{
    DiagResult, DiagnosticCode, DiagnosticSource, Level,
    diags::errors::{ErrorCode, build_remir_error},
};
use calsc_utils::pos::FilePosition;
use remir::errs::RemirResult;

pub trait CalscinRemirResult<K> {
    fn convert(self, start: FilePosition, end: FilePosition) -> DiagResult<K>;
    fn convert_source<S: DiagnosticSource>(self, source: &S) -> DiagResult<K>;
}

impl<K> CalscinRemirResult<K> for RemirResult<K> {
    fn convert(self, start: FilePosition, end: FilePosition) -> DiagResult<K> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(build_remir_error(&e, start, end).into()),
        }
    }

    fn convert_source<S: DiagnosticSource>(self, source: &S) -> DiagResult<K> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                return Err(source
                    .make_diagnostic_simple(
                        DiagnosticCode::new(Level::Error, ErrorCode::RemirError as usize),
                        format!("ReMIR error: {}", e),
                        None,
                        vec![],
                        vec![],
                        vec![],
                    )
                    .into());
            }
        }
    }
}

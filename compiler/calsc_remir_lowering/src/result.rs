use calsc_diagnostics::{DiagResult, diags::errors::build_remir_error};
use calsc_utils::pos::FilePosition;
use remir::errs::RemirResult;

pub trait CalscinRemirResult<K> {
    fn convert(self, start: FilePosition, end: FilePosition) -> DiagResult<K>;
}

impl<K> CalscinRemirResult<K> for RemirResult<K> {
    fn convert(self, start: FilePosition, end: FilePosition) -> DiagResult<K> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(build_remir_error(&e, start, end).into()),
        }
    }
}

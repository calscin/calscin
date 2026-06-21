use std::fmt::Display;

use crate::{Diagnostic, DiagnosticCode, DiagnosticSource};

enum WarningCode {
    UselessCast,
}

pub fn build_useless_cast<T: Display, S: DiagnosticSource>(
    from: &T,
    into: &T,
    source: &S,
) -> Diagnostic {
    source.make_diagnostic_simple(
        DiagnosticCode::new(crate::Level::Warning, WarningCode::UselessCast as usize),
        format!("cast from {} into {} is useless", from, into),
        None,
        vec![],
        vec![format!(
            "{} can directly be transmutated into {} without a cast",
            from, into
        )],
        vec!["remove this cast".into()],
    )
}

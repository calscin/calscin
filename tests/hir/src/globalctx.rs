#[cfg(test)]
use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_hir::globalctx::{GlobalContext, key::GlobalContextKey, vals::GlobalContextValue};

#[cfg(test)]
use calsc_typing::base::{BaseType, kind::BaseTypeKind};

#[cfg(test)]
use calsc_utils::pos::FilePosition;

#[test]
fn globalctx_entry_retrival_test() {
    let origin = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default());

    let mut globalctx = GlobalContext::new();

    let key = GlobalContextKey::new("test".into());

    let type_entry = BaseType::new(BaseTypeKind::Boolean);
    let entry = GlobalContextValue::Type(type_entry.clone());

    globalctx
        .append(key.clone(), entry, &origin)
        .unwrap_cleanly();

    let entry = globalctx.get_entry(key, &origin).unwrap_cleanly();

    assert_eq!(entry.as_type(&origin).unwrap_cleanly(), type_entry);
}

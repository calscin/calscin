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

#[test]
fn globalctx_entry_retrival_none_test() {
    let origin = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default());

    let globalctx = GlobalContext::new();

    let key = GlobalContextKey::new("test".into());
    let _ = globalctx.get_entry(key, &origin).unwrap_err();
}

#[test]
fn globalctx_type_mutation_test() {
    let origin = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default());

    let mut globalctx = GlobalContext::new();

    let key = GlobalContextKey::new("test".into());

    let type_entry = BaseType::new(BaseTypeKind::Boolean);
    let second_type_entry = BaseType::new(BaseTypeKind::Floating { signed: true });
    let entry = GlobalContextValue::Type(type_entry.clone());

    globalctx
        .append(key.clone(), entry, &origin)
        .unwrap_cleanly();

    assert_eq!(
        globalctx
            .get_entry(key.clone(), &origin)
            .unwrap_cleanly()
            .as_type(&origin)
            .unwrap_cleanly(),
        type_entry.clone()
    );

    // Mutation

    globalctx
        .mutate_entry(
            key.clone(),
            |val| {
                val.mutate_type(|ty| *ty = second_type_entry.clone(), &origin)
                    .unwrap_cleanly()
            },
            &origin,
        )
        .unwrap_cleanly();

    // Second assert

    assert_eq!(
        globalctx
            .get_entry(key, &origin)
            .unwrap_cleanly()
            .as_type(&origin)
            .unwrap_cleanly(),
        second_type_entry
    );
}

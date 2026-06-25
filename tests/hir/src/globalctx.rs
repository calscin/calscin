#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_hir::file::HIRFileContext;

#[cfg(test)]
use calsc_hir::globalctx::{GlobalContext, key::GlobalContextKey, vals::GlobalContextValue};

#[cfg(test)]
use calsc_modules::visibility::Visibility;

#[cfg(test)]
use calsc_typing::types::primitive::PrimitiveType;

#[cfg(test)]
use calsc_utils::pos::FilePosition;

#[test]
fn globalctx_entry_retrival_test() {
    let origin = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default());

    let mut globalctx = GlobalContext::new();

    let key = GlobalContextKey::new("test".into());

    let entry = GlobalContextValue::Type(PrimitiveType::Boolean);

    globalctx
        .append(key.clone(), entry, Visibility::Public, &origin)
        .unwrap_cleanly();

    let entry = globalctx
        .get_entry_no_visibility(key, &origin)
        .unwrap_cleanly();

    assert_eq!(
        entry.as_type(&origin).unwrap_cleanly(),
        PrimitiveType::Boolean
    );
}

#[test]
fn globalctx_entry_retrival_none_test() {
    let origin = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default());

    let globalctx = GlobalContext::new();

    let key = GlobalContextKey::new("test".into());
    let _ = globalctx.get_entry_no_visibility(key, &origin).unwrap_err();
}

#[test]
fn globalctx_type_mutation_test() {
    let origin = PosDiagnosticSource::new(FilePosition::default(), FilePosition::default());

    let mut globalctx = GlobalContext::new();

    let key = GlobalContextKey::new("test".into());

    let type_entry = PrimitiveType::Boolean;

    let second_entry_type = PrimitiveType::Float;

    let entry = GlobalContextValue::Type(type_entry.clone());

    globalctx
        .append(key.clone(), entry, Visibility::Public, &origin)
        .unwrap_cleanly();

    assert_eq!(
        globalctx
            .get_entry(
                key.clone(),
                &HIRFileContext::new(PathBuf::from("")).current_module,
                &origin
            )
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
                val.mutate_type(|ty| *ty = second_entry_type.clone(), &origin)
                    .unwrap_cleanly()
            },
            &origin,
        )
        .unwrap_cleanly();

    // Second assert

    assert_eq!(
        globalctx
            .get_entry_no_visibility(key, &origin)
            .unwrap_cleanly()
            .as_type(&origin)
            .unwrap_cleanly(),
        second_entry_type
    );
}

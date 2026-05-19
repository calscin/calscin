use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};
use calsc_typing::{
    base::{BaseType, BaseTypeInstance, kind::BaseTypeKind},
    func::{DeclBlockAffectedType, TypedFunction},
    tree::Type,
};

#[test]
pub fn function_append_retrival_test() {
    let source = PosDiagnosticSource::new(Default::default(), Default::default());

    let mut base = BaseType::new(BaseTypeKind::Boolean);

    let instance = BaseTypeInstance::new(base.clone(), vec![], vec![]);

    base.add_function(
        "test_function".into(),
        TypedFunction::new(
            "test_function".into(),
            vec![Type::Base(instance.clone())],
            Some(Type::Base(instance.clone())),
        ),
        &source,
    )
    .unwrap_cleanly();

    assert!(base.has_function(
        "test_function".into(),
        (
            vec![Type::Base(instance.clone())],
            Some(Type::Base(instance.clone()))
        )
    ));
}

#[test]
pub fn no_function_retrival_test() {
    let base = BaseType::new(BaseTypeKind::Boolean);

    assert!(!base.has_function("test".into(), (vec![], None)));
}

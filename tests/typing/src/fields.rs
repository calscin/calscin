use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};
use calsc_typing::{
    FieldHavingType, MutableFieldHavingType,
    base::{BaseType, BaseTypeInstance, kind::BaseTypeKind, structs::BaseStructContainer},
    tree::Type,
};

#[test]
fn test_field_retrival_no_struct() {
    let base = BaseType::new(BaseTypeKind::Boolean);

    assert!(!base.has_field("test".into()));
}

#[test]
fn test_field_retrival_struct() {
    let source = PosDiagnosticSource::new(Default::default(), Default::default());

    let field_ty = Type::Base(BaseTypeInstance::new(
        BaseType::new(BaseTypeKind::Integer { signed: true }),
        vec![12],
        vec![],
    ));
    let mut container = BaseStructContainer::new("test".into());

    container
        .add_field("test_field".into(), field_ty.clone(), &source)
        .unwrap_cleanly();

    let ty = BaseType::new(BaseTypeKind::Struct(container));

    assert!(ty.has_field("test_field".into()));
    assert_eq!(ty.get_field_type("test_field".into()), field_ty);
}

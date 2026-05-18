use calsc_typing::base::{BaseType, kind::BaseTypeKind};

#[test]
fn test_base_type_required_size_params() {
    assert_eq!(
        BaseTypeKind::Integer { signed: true }.get_required_size_parameters(),
        1
    );

    assert_eq!(
        BaseTypeKind::Floating { signed: true }.get_required_size_parameters(),
        1
    );

    assert_eq!(BaseTypeKind::Boolean.get_required_size_parameters(), 0);
}

#[test]
fn test_base_type_creation() {
    let _ = BaseType::new(BaseTypeKind::Integer { signed: false }, vec![32]); // Create a 32 bit unsigned integer type
}

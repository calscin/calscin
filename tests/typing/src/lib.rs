#[cfg(test)]
use calsc_diagnostics::{panics::PanicDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_typing::{
    ctx::TypeCtx,
    types::{SizeParameter, TypeKind, primitive::PrimitiveType},
};

pub mod fields;

#[test]
fn test_base_type_required_size_params() {
    assert!(PrimitiveType::Int(true).requires_size_parameter());
    assert!(PrimitiveType::Float.requires_size_parameter());
    assert!(!PrimitiveType::Boolean.requires_size_parameter());
}

#[test]
fn test_base_type_creation() {
    let mut type_ctx = TypeCtx::new();
    let fake_source = PanicDiagnosticSource();

    let _ = TypeKind::new_primitive(
        PrimitiveType::Int(true),
        SizeParameter(40),
        &mut type_ctx,
        &fake_source,
    )
    .unwrap_cleanly();
}

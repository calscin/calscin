#[cfg(test)]
use calsc_diagnostics::{panics::PanicDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_typing::TypingInterner;

#[cfg(test)]
use calsc_typing::types::{SizeParameter, TypeKind, primitive::PrimitiveType};

pub mod fields;

#[test]
fn test_base_type_required_size_params() {
    assert!(PrimitiveType::Int(true).requires_size_parameter());
    assert!(PrimitiveType::Float.requires_size_parameter());
    assert!(!PrimitiveType::Boolean.requires_size_parameter());
}

#[test]
fn test_base_type_creation() {
    let mut type_interner = TypingInterner::new();

    let fake_source = PanicDiagnosticSource();

    let _ = TypeKind::new_primitive(
        PrimitiveType::Int(true),
        SizeParameter(40),
        vec![],
        &mut type_interner,
        &fake_source,
    )
    .unwrap_cleanly();
}

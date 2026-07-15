#[cfg(test)]
use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_modules::path::ModulePath;

#[cfg(test)]
use calsc_typing::TypingInterner;

#[cfg(test)]
use calsc_typing::{
    ctx::TypeCtx,
    traits::FieldedType,
    types::{
        TypeKind,
        primitive::PrimitiveType,
        structs::{NamedField, StructContainer},
    },
};

#[test]
fn test_field_retrival_no_struct() {
    let ctx = TypeCtx::new();
    let interner = TypingInterner::new();

    let base = TypeKind::make_bool_type();

    assert!(!base.has_field(&"test".into(), &ctx, &interner));
}

#[test]
fn test_field_retrival_struct() {
    let source = PosDiagnosticSource::new(Default::default(), Default::default());
    let type_ctx = TypeCtx::new();
    let mut interner = TypingInterner::new();

    let field_ty = TypeKind::make_int_type(true, 12);

    let mut container = StructContainer::new("test".into(), ModulePath::new("".into(), vec![]));

    container
        .fields
        .append_named(NamedField("test_field".into(), field_ty.clone()), &source)
        .unwrap_cleanly();

    let container = interner.struct_container_arena.append(container);

    let ty = PrimitiveType::Struct(container);

    assert!(ty.has_field(&"test_field".into(), &type_ctx, &interner));
    assert_eq!(
        ty.get_field_safe(&"test_field".into(), &type_ctx, &interner, &source)
            .unwrap_cleanly(),
        field_ty
    );
}

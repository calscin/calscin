#[cfg(test)]
use calsc_diagnostics::{PosDiagnosticSource, result::CalscinResult};

#[cfg(test)]
use calsc_modules::path::ModulePath;

use calsc_typing::allocs::STRUCT_CONTAINER_ALLOC;
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

    let base = TypeKind::make_bool_type();

    assert!(!base.has_field(&"test".into(), &ctx));
}

#[test]
fn test_field_retrival_struct() {
    let source = PosDiagnosticSource::new(Default::default(), Default::default());
    let type_ctx = TypeCtx::new();

    let field_ty = TypeKind::make_int_type(true, 12);

    let mut container = StructContainer::new("test".into(), ModulePath::new("".into(), vec![]));

    container
        .fields
        .append_named(NamedField("test_field".into(), field_ty.clone()), &source)
        .unwrap_cleanly();

    let container = STRUCT_CONTAINER_ALLOC.with(|f| f.borrow_mut().append(container));

    let ty = PrimitiveType::Struct(container);

    assert!(ty.has_field(&"test_field".into(), &type_ctx));
    assert_eq!(
        ty.get_field_safe(&"test_field".into(), &type_ctx, &source)
            .unwrap_cleanly(),
        field_ty
    );
}

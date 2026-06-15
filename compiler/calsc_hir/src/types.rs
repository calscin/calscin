//! Builder utilities for types

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_expected_size_specifiers_error, build_expected_type_parameters_error},
    result::CalscinResult,
};
use calsc_typing::{
    base::{BaseType, instance::BaseTypeInstance},
    params::TypeParameterHaving,
    tree::Type,
};

use crate::{HIRContext, globalctx::key::GlobalContextKey};

pub fn make_int_type<K: DiagnosticSource>(
    signed: bool,
    size: usize,
    origin: &K,
    ctx: &HIRContext,
) -> Type {
    let mut key = GlobalContextKey::new("s".into());

    if !signed {
        key = GlobalContextKey::new("u".into());
    }

    let base_type = ctx
        .scope
        .get_entry(key, origin)
        .unwrap_cleanly()
        .as_type(origin)
        .unwrap_cleanly();

    Type::Base(BaseTypeInstance::new(base_type, vec![size], vec![]))
}

pub fn make_float_type<K: DiagnosticSource>(
    signed: bool,
    size: usize,
    origin: &K,
    ctx: &HIRContext,
) -> Type {
    let mut key = GlobalContextKey::new("f".into());

    if !signed {
        key = GlobalContextKey::new("uf".into());
    }

    let base_type = ctx
        .scope
        .get_entry(key, origin)
        .unwrap_cleanly()
        .as_type(origin)
        .unwrap_cleanly();

    Type::Base(BaseTypeInstance::new(base_type, vec![size], vec![]))
}

pub fn make_bool_type<K: DiagnosticSource>(origin: &K, ctx: &HIRContext) -> Type {
    let key = GlobalContextKey::new("bool".into());

    let base_type = ctx
        .scope
        .get_entry(key, origin)
        .unwrap_cleanly()
        .as_type(origin)
        .unwrap_cleanly();

    Type::Base(BaseTypeInstance::new(base_type, vec![], vec![]))
}

pub fn make_string_type<K: DiagnosticSource>(origin: &K, ctx: &HIRContext) -> Type {
    let key = GlobalContextKey::new("str".into());

    let base_type = ctx
        .scope
        .get_entry(key, origin)
        .unwrap_cleanly()
        .as_type(origin)
        .unwrap_cleanly();

    Type::Base(BaseTypeInstance::new(base_type, vec![], vec![]))
}

pub fn make_char_type<K: DiagnosticSource>(origin: &K, ctx: &HIRContext) -> Type {
    let key = GlobalContextKey::new("char".into());

    let base_type = ctx
        .scope
        .get_entry(key, origin)
        .unwrap_cleanly()
        .as_type(origin)
        .unwrap_cleanly();

    Type::Base(BaseTypeInstance::new(base_type, vec![], vec![]))
}

pub fn safely_make_type_instance<K: DiagnosticSource>(
    ty: BaseType,
    size_specifiers: Vec<usize>,
    type_parameters: Vec<Type>,
    origin: &K,
) -> DiagResult<BaseTypeInstance> {
    if ty.kind.get_required_size_parameters() != size_specifiers.len() {
        return Err(build_expected_size_specifiers_error(
            &ty.kind.get_required_size_parameters(),
            &size_specifiers.len(),
            origin,
        )
        .into());
    }

    if ty.get_type_parameter_count() != type_parameters.len() {
        return Err(build_expected_type_parameters_error(
            &ty.get_type_parameter_count(),
            &type_parameters.len(),
            origin,
        )
        .into());
    }

    Ok(BaseTypeInstance::new(ty, size_specifiers, type_parameters))
}

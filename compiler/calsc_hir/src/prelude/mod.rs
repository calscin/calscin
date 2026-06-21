//! The environment in which the real barebones are loaded. This is were compiler builtins are loaded into the HIR

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::{path::ModulePath, visibility::Visibility};
use calsc_typing::base::{BaseType, kind::BaseTypeKind};

use crate::globalctx::{GlobalContext, key::GlobalContextKey, vals::GlobalContextValue};

/// Applies the HIR prelude to the given scope
pub fn apply_prelude<K: DiagnosticSource>(scope: &mut GlobalContext, origin: &K) -> DiagPossible {
    let module_path = ModulePath::new_prelude_path(vec!["types".into()]);

    let bool_type = GlobalContextValue::new_type(BaseType::new(BaseTypeKind::Boolean));

    let signed_integer_type =
        GlobalContextValue::new_type(BaseType::new(BaseTypeKind::Integer { signed: true }));
    let unsigned_integer_type =
        GlobalContextValue::new_type(BaseType::new(BaseTypeKind::Integer { signed: true }));

    let signed_float_type =
        GlobalContextValue::new_type(BaseType::new(BaseTypeKind::Floating { signed: true }));

    let unsigned_float_type =
        GlobalContextValue::new_type(BaseType::new(BaseTypeKind::Floating { signed: false }));

    let string_type = GlobalContextValue::new_type(BaseType::new(BaseTypeKind::String));
    let char_type = GlobalContextValue::new_type(BaseType::new(BaseTypeKind::Char));

    let size_type = GlobalContextValue::new_type(BaseType::new(BaseTypeKind::Size));

    scope.append(
        GlobalContextKey::new("bool".into()),
        bool_type,
        Visibility::Uncopiable,
        origin,
    )?;

    scope.append(
        GlobalContextKey::new("s".into()).module_path(module_path.clone()),
        signed_integer_type,
        Visibility::Uncopiable,
        origin,
    )?;

    scope.append(
        GlobalContextKey::new("u".into()).module_path(module_path.clone()),
        unsigned_integer_type,
        Visibility::Uncopiable,
        origin,
    )?;

    scope.append(
        GlobalContextKey::new("f".into()).module_path(module_path.clone()),
        signed_float_type,
        Visibility::Uncopiable,
        origin,
    )?;

    scope.append(
        GlobalContextKey::new("uf".into()).module_path(module_path.clone()),
        unsigned_float_type,
        Visibility::Uncopiable,
        origin,
    )?;

    scope.append(
        GlobalContextKey::new("str".into()).module_path(module_path.clone()),
        string_type,
        Visibility::Uncopiable,
        origin,
    )?;
    scope.append(
        GlobalContextKey::new("char".into()).module_path(module_path.clone()),
        char_type,
        Visibility::Uncopiable,
        origin,
    )?;

    scope.append(
        GlobalContextKey::new("size".into()).module_path(module_path.clone()),
        size_type,
        Visibility::Uncopiable,
        origin,
    )?;

    Ok(())
}

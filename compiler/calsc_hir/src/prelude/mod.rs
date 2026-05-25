//! The environment in which the real barebones are loaded. This is were compiler builtins are loaded into the HIR

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_typing::base::{BaseType, kind::BaseTypeKind};

use crate::globalctx::{GlobalContext, key::GlobalContextKey, vals::GlobalContextValue};

/// Applies the HIR prelude to the given scope
pub fn apply_prelude<K: DiagnosticSource>(scope: &mut GlobalContext, origin: &K) -> DiagPossible {
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

    scope.append(GlobalContextKey::new("bool".into()), bool_type, origin)?;

    scope.append(
        GlobalContextKey::new("s".into()),
        signed_integer_type,
        origin,
    )?;

    scope.append(
        GlobalContextKey::new("u".into()),
        unsigned_integer_type,
        origin,
    )?;

    scope.append(GlobalContextKey::new("f".into()), signed_float_type, origin)?;

    scope.append(
        GlobalContextKey::new("uf".into()),
        unsigned_float_type,
        origin,
    )?;

    scope.append(GlobalContextKey::new("str".into()), string_type, origin)?;
    scope
        .append(GlobalContextKey::new("char".into()), char_type, origin)?;
        .Ok(())
}

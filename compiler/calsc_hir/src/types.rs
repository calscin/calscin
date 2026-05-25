//! Builder utilities for types

use calsc_diagnostics::{DiagnosticSource, result::CalscinResult};
use calsc_typing::{base::instance::BaseTypeInstance, tree::Type};

use crate::{HIR_CONTEXT, globalctx::key::GlobalContextKey};

pub fn make_int_type<K: DiagnosticSource>(signed: bool, size: usize, origin: &K) -> Type {
    let mut key = GlobalContextKey::new("s".into());

    if !signed {
        key = GlobalContextKey::new("u".into());
    }

    let base_type = HIR_CONTEXT.with_borrow(|f| {
        f.scope
            .get_entry(key, origin)
            .unwrap_cleanly()
            .as_type(origin)
            .unwrap_cleanly()
    });

    Type::Base(BaseTypeInstance::new(base_type, vec![size], vec![]))
}

pub fn make_float_type<K: DiagnosticSource>(signed: bool, size: usize, origin: &K) -> Type {
    let mut key = GlobalContextKey::new("f".into());

    if !signed {
        key = GlobalContextKey::new("uf".into());
    }

    let base_type = HIR_CONTEXT.with_borrow(|f| {
        f.scope
            .get_entry(key, origin)
            .unwrap_cleanly()
            .as_type(origin)
            .unwrap_cleanly()
    });

    Type::Base(BaseTypeInstance::new(base_type, vec![size], vec![]))
}

pub fn make_bool_type<K: DiagnosticSource>(origin: &K) -> Type {
    let key = GlobalContextKey::new("bool".into());

    let base_type = HIR_CONTEXT.with_borrow(|f| {
        f.scope
            .get_entry(key, origin)
            .unwrap_cleanly()
            .as_type(origin)
            .unwrap_cleanly()
    });

    Type::Base(BaseTypeInstance::new(base_type, vec![], vec![]))
}

pub fn make_string_type<K: DiagnosticSource>(origin: &K) -> Type {
    let key = GlobalContextKey::new("str".into());

    let base_type = HIR_CONTEXT.with_borrow(|f| {
        f.scope
            .get_entry(key, origin)
            .unwrap_cleanly()
            .as_type(origin)
            .unwrap()
    });

    Type::Base(BaseTypeInstance::new(base_type, vec![], vec![]))
}

pub fn make_char_type<K: DiagnosticSource>(origin: &K) -> Type {
    let key = GlobalContextKey::new("char".into());

    let base_type = HIR_CONTEXT.with_borrow(|f| {
        f.scope
            .get_entry(key, origin)
            .unwrap_cleanly()
            .as_type(origin)
            .unwrap()
    });

    Type::Base(BaseTypeInstance::new(base_type, vec![], vec![]))
}

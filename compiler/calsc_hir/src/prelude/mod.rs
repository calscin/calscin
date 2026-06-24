//! The environment in which the real barebones are loaded. This is were compiler builtins are loaded into the HIR

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::{path::ModulePath, visibility::Visibility};
use calsc_typing::{traits::PreludeApplier, types::primitive::PrimitiveType};
use calsc_utils::hash::HashedString;

use crate::globalctx::{GlobalContext, key::GlobalContextKey, vals::GlobalContextValue};

impl PreludeApplier for GlobalContext {
    fn register_type<K: DiagnosticSource>(
        &mut self,
        name: HashedString,
        ty: PrimitiveType,
        source: &K,
    ) -> DiagPossible {
        let module_path = ModulePath::new_prelude_path(vec!["types".into()]);

        self.append(
            GlobalContextKey::new(name).module_path(module_path),
            GlobalContextValue::Type(ty),
            Visibility::Uncopiable,
            source,
        )?;

<<<<<<< HEAD
        Ok(())
    }
=======
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
>>>>>>> master
}

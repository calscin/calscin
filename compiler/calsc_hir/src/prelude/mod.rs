//! The environment in which the real barebones are loaded. This is were compiler builtins are loaded into the HIR

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::{path::ModulePath, visibility::Visibility};
use calsc_typing_v2::{traits::PreludeApplier, types::primitive::PrimitiveType};
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

        Ok(())
    }
}

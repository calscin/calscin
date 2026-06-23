use std::collections::HashMap;

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::path::ModulePath;
use calsc_typing_v2::{traits::PreludeApplier, types::primitive::PrimitiveType};
use calsc_utils::hash::HashedString;

pub struct ResolvedTypeCache {
    pub map: HashMap<ModulePath, PrimitiveType>,
}

impl ResolvedTypeCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl PreludeApplier for ResolvedTypeCache {
    fn register_type<K: DiagnosticSource>(
        &mut self,
        name: HashedString,
        ty: PrimitiveType,
        _source: &K,
    ) -> DiagPossible {
        self.map
            .insert(ModulePath::new_module_tree_prelude_path(vec![name]), ty);
        Ok(())
    }
}

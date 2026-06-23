use std::collections::HashMap;

use calsc_modules::path::ModulePath;
use calsc_typing_v2::types::primitive::PrimitiveType;

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

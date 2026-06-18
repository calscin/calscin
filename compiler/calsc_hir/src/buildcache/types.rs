use std::collections::HashMap;

use calsc_modules::path::ModulePath;
use calsc_typing::base::BaseType;

pub struct ResolvedTypeCache {
    pub map: HashMap<ModulePath, BaseType>,
}

impl ResolvedTypeCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

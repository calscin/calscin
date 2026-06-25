//! Declarations for type parameters

use std::collections::HashMap;

use calsc_utils::hash::HashedString;

/// This is a safe handle from a type parameter stored inside of a [`TypeParamCtx`] this enforces that type parameters go trough the expected path.
pub struct TypeParameterId(usize);

pub struct TypeParamCtx {
    params: HashMap<HashedString, HeldTypeParam>,
}

struct HeldTypeParam {
    name: HashedString,
    id: usize,
}

impl TypeParamCtx {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }
}

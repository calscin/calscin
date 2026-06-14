//! Definitions for the Global context keys.

use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use calsc_modules::path::ModulePath;
use calsc_typing::base::BaseType;
use calsc_utils::hash::HashedString;

/// The key to an entry in the global ctx
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Eq, Clone)]
pub struct GlobalContextKey {
    pub name: HashedString,

    pub module_path: ModulePath,

    pub type_name: Option<BaseType>,
}

impl GlobalContextKey {
    /// Creates a new [`GlobalContextKey`] based on the given element name.
    pub fn new(name: HashedString) -> Self {
        Self {
            name,
            module_path: Default::default(),
            type_name: None,
        }
    }

    #[inline(always)]
    pub fn associated_type(mut self, type_name: BaseType) -> Self {
        self.type_name = Some(type_name);

        self
    }

    #[inline(always)]
    pub fn module_path(mut self, module_path: ModulePath) -> Self {
        self.module_path = module_path;
        self
    }
}

impl Hash for GlobalContextKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(1); // Marker for HIR type values to avoid collisions with hashes from HashedString
        self.module_path.hash(hasher);
        hasher.write_usize(self.type_name.is_some() as usize);

        if self.type_name.is_some() {
            self.type_name.clone().unwrap().hash(hasher);
        }

        self.name.hash(hasher);
    }
}

impl Display for GlobalContextKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.type_name.is_none() {
            write!(f, "{}::{}", self.module_path, *self.name)
        } else {
            write!(
                f,
                "{}::{}::{}",
                self.module_path,
                self.type_name.clone().unwrap(),
                *self.name
            )
        }
    }
}

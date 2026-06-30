//! Definitions for the Global context keys.

use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use calsc_modules::path::ModulePath;

use calsc_utils::hash::HashedString;

/// The key to an entry in the global ctx
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Eq, Clone)]
pub struct GlobalContextKey {
    pub name: HashedString,

    pub module_path: ModulePath,
    pub associated_type: Option<Box<GlobalContextKey>>,
}

impl GlobalContextKey {
    /// Creates a new [`GlobalContextKey`] based on the given element name.
    pub fn new(name: HashedString) -> Self {
        Self {
            name,
            module_path: Default::default(),
            associated_type: None,
        }
    }

    #[inline(always)]
    pub fn module_path(mut self, module_path: ModulePath) -> Self {
        self.module_path = module_path;
        self
    }

    pub fn associated_type(mut self, associated_type: GlobalContextKey) -> Self {
        self.associated_type = Some(Box::new(associated_type));
        self
    }
}

impl Hash for GlobalContextKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(1); // Marker for HIR type values to avoid collisions with hashes from HashedString
        self.module_path.hash(hasher);

        hasher.write_usize(self.associated_type.is_some() as usize);

        if self.associated_type.is_some() {
            self.associated_type.as_ref().unwrap().hash(hasher);
        }

        self.name.hash(hasher);
    }
}

impl Display for GlobalContextKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.module_path.is_empty() {
            write!(f, "{}::", self.module_path)?;
        }

        if self.associated_type.is_some() {
            write!(f, "{}::", self.associated_type.as_ref().unwrap())?;
        }

        write!(f, "{}", *self.name)
    }
}

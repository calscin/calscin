//! Definitions for the Global context keys.

use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use calsc_utils::hash::HashedString;

/// The key to an entry in the global ctx
#[derive(PartialEq, Eq, Clone)]
pub struct GlobalContextKey {
    pub name: HashedString,
}

impl GlobalContextKey {
    /// Creates a new [`GlobalContextKey`] based on the given element name.
    pub fn new(name: HashedString) -> Self {
        Self { name }
    }
}

impl Hash for GlobalContextKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(1); // Marker for HIR type values to avoid collisions with hashes from HashedString
        hasher.write_u64(self.name.hash());
    }
}

impl Display for GlobalContextKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self.name)
    }
}

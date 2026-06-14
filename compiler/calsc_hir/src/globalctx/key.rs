//! Definitions for the Global context keys.

use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use calsc_typing::base::BaseType;
use calsc_utils::hash::HashedString;

/// The key to an entry in the global ctx
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(PartialEq, Eq, Clone)]
pub struct GlobalContextKey {
    pub name: HashedString,
    pub type_name: Option<BaseType>,
}

impl GlobalContextKey {
    /// Creates a new [`GlobalContextKey`] based on the given element name.
    pub fn new(name: HashedString) -> Self {
        Self {
            name,
            type_name: None,
        }
    }

    /// Creates a new [`GlobalContextKey`] based on the given element name and type
    pub fn new_typed(name: HashedString, type_name: BaseType) -> Self {
        Self {
            name,
            type_name: Some(type_name),
        }
    }
}

impl Hash for GlobalContextKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(1); // Marker for HIR type values to avoid collisions with hashes from HashedString
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
            write!(f, "{}", *self.name)
        } else {
            write!(f, "{}::{}", self.type_name.clone().unwrap(), *self.name)
        }
    }
}

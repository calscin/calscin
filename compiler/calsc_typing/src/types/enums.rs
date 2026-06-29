use std::collections::HashMap;

use calsc_modules::path::ModulePath;
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::types::structs::FieldContainer;

/// The container for an enum type
pub struct EnumContainer {
    pub name: HashedString,
    pub module: ModulePath,

    pub entries: HashMap<HashedString, EnumEntryContainer>,
    pub type_parameters: Vec<HashedString>,
}

/// The container for an enum entry type
pub struct EnumEntryContainer {
    pub name: HashedString,

    pub fields: FieldContainer,
    pub parent: ArenaHandle,
}

impl EnumEntryContainer {
    pub fn new(name: HashedString, parent: ArenaHandle) -> Self {
        Self {
            name,
            fields: FieldContainer::new(),
            parent,
        }
    }
}

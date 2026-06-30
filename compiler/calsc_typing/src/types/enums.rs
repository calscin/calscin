use std::collections::HashMap;

use calsc_modules::path::ModulePath;
use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::types::{TypeKind, structs::FieldContainer};

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

impl EnumContainer {
    pub fn new(name: HashedString, module: ModulePath) -> Self {
        Self {
            name,
            module,
            entries: HashMap::new(),
            type_parameters: vec![],
        }
    }

    pub fn get_marker_type(&self) -> TypeKind {
        let bits_needed = usize::BITS - (self.entries.len() - 1).leading_zeros();

        TypeKind::make_int_type(false, bits_needed as usize)
    }
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

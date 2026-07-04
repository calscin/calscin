use std::{collections::HashMap, path::PathBuf};

use calsc_utils::{alloc::arena::ArenaHandle, hash::HashedString};

use crate::treev2::imports::ImportFilter;

/// The type of module
#[cfg_attr(feature = "debug", derive(Debug))]
pub enum TreeModuleType {
    /// Represents a module that envelops an entire file
    File,

    /// Represents a module that envelops a part of a file
    Inner,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone)]
pub struct TreeModule {
    /// The name of the module.
    pub name: HashedString,

    /// The children contained within the module.
    /// Can either be:
    /// - a type
    /// - a function
    /// - another module
    pub children: HashMap<HashedString, ArenaHandle>,

    /// The imports inside of the module.
    /// This represents a table of:
    /// - `imported element name / path -> true full path inside of the package`
    pub imports: Vec<ImportFilter>,

    /// The file at the origin of the module
    pub path: PathBuf,
}

impl TreeModule {
    pub fn new(name: HashedString, file_path: PathBuf) -> Self {
        Self {
            name,
            children: HashMap::new(),
            imports: vec![],
            path: file_path,
        }
    }
}

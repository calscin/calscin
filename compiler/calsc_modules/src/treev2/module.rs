use std::{collections::HashMap, path::PathBuf};

use calsc_utils::hash::HashedString;

use crate::{path::ModulePath, treev2::entry::TreeEntry};

/// The type of module
pub enum TreeModuleType {
    /// Represents a module that envelops an entire file
    File,

    /// Represents a module that envelops a part of a file
    Inner,
}

pub struct TreeModule {
    /// The name of the module.
    pub name: HashedString,

    /// The children contained within the module.
    /// Can either be:
    /// - a type
    /// - a function
    /// - another module
    pub children: HashMap<HashedString, TreeEntry>,

    /// The imports inside of the module.
    /// This represents a table of:
    /// - `imported element name / path -> true full path inside of the package`
    pub imports: HashMap<ModulePath, ModulePath>,

    /// The file at the origin of the module
    pub path: PathBuf,
}

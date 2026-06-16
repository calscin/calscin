use std::collections::HashMap;

use calsc_typing::tree::Type;
use calsc_utils::hash::HashedString;

#[derive(Debug)]
pub enum ModuleTreeEntry {
    Function(Type, Vec<(HashedString, Type)>),
    Module(TreeModule),
}

/// A module contained inside of the module tree
#[derive(Debug)]
pub struct TreeModule {
    pub name: HashedString,
    pub children: HashMap<HashedString, ModuleTreeEntry>,
}

impl ModuleTreeEntry {
    /// Checks if the given [`ModuleTreeEntry`] can contain children.
    /// This is used for traversing
    pub fn has_children(&self) -> bool {
        match self {
            Self::Module(_) => true,

            _ => false,
        }
    }
}

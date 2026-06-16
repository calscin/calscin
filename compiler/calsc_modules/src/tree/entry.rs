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
    pub children: Vec<ModuleTreeEntry>,
}

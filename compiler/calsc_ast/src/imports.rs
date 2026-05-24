use calsc_utils::hash::HashedString;

/// The kind of import statement
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ImportKind {
    /// Imports the whole module with the module namespace
    Whole,

    /// Imports only specific items but put them directly inside of the HIR tree of the current file inside of the namespace.
    Items(Vec<HashedString>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum ImportModule {
    /// The standart library
    Std,

    /// The packiage with it's package
    Package(HashedString),
}

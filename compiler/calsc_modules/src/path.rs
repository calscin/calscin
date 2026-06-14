use std::hash::Hash;

use calsc_utils::hash::HashedString;

/// Represents a path to a module
pub struct ModulePath {
    /// The package name of the module.
    pub package: HashedString,

    /// The path of modules to go to the module.
    /// Each entry represents a module to walk trough
    pub path: Vec<HashedString>,
}

impl ModulePath {
    /// Creates a new [`ModulePath`]
    pub fn new(package: HashedString, path: Vec<HashedString>) -> Self {
        Self { package, path }
    }

    pub fn is_prelude(&self) -> bool {
        self.package == "prelude".into()
    }
}

impl Hash for ModulePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.is_prelude() {
            return;
        }

        self.package.hash(state);
        state.write_usize(self.path.len());

        let _ = self.path.iter().map(|entry| entry.hash(state));
    }
}

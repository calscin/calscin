use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use calsc_utils::hash::HashedString;

pub struct PackageLessModulePath(Vec<HashedString>);

/// Represents a path to a module
#[derive(Clone)]
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

    /// Creates a new [`ModulePath`]
    pub fn new_prelude_path(path: Vec<HashedString>) -> Self {
        Self {
            package: "prelude".into(),
            path,
        }
    }

    pub fn new_module_tree_prelude_path(path: Vec<HashedString>) -> Self {
        Self {
            package: "tree_prelude".into(),
            path,
        }
    }

    pub fn is_prelude(&self) -> bool {
        self.package == "prelude".into()
    }

    pub fn is_empty(&self) -> bool {
        self.package.is_empty() && self.path.is_empty()
    }

    pub fn append(&mut self, path: ModulePath) {
        if !path.package.is_empty() {
            self.path.push(path.package);
        }

        for entry in path.path {
            self.path.push(entry);
        }
    }

    pub fn append_single_bit(&mut self, bit: HashedString) {
        if self.package.is_empty() {
            self.package = bit;
            return;
        }

        self.path.push(bit);
    }

    pub fn get(&self, ind: usize) -> HashedString {
        if ind == 0 {
            self.package.clone()
        } else {
            self.path[ind - 1].clone()
        }
    }

    pub fn get_ref<'a>(&'a self, ind: usize) -> &'a HashedString {
        if ind == 0 {
            &self.package
        } else {
            &self.path[ind - 1]
        }
    }

    pub fn get_size(&self) -> usize {
        self.path.len() + 1
    }

    pub fn last(&self) -> HashedString {
        if self.path.is_empty() {
            self.package.clone()
        } else {
            self.path[self.path.len() - 1].clone()
        }
    }

    pub fn everything_but_last(&self) -> ModulePath {
        if self.path.len() <= 1 {
            ModulePath::new(self.package.clone(), vec![])
        } else {
            ModulePath::new(
                self.package.clone(),
                self.path[0..self.path.len() - 1].to_vec(),
            )
        }
    }
}

impl Default for ModulePath {
    fn default() -> Self {
        Self {
            package: "".into(),
            path: vec![],
        }
    }
}

impl PartialEq for ModulePath {
    fn eq(&self, other: &Self) -> bool {
        if self.is_prelude() || other.is_prelude() {
            return true;
        }

        return self.package == other.package && self.path == other.path;
    }
}

impl Eq for ModulePath {}

impl Hash for ModulePath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.is_prelude() || self.package == "".into() {
            return;
        }

        self.package.hash(state);
        state.write_usize(self.path.len());

        let _ = self.path.iter().map(|entry| entry.hash(state));
    }
}

impl Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.package)?;

        for path in &self.path {
            write!(f, "::{}", path)?;
        }

        Ok(())
    }
}

impl Debug for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.package)?;

        for path in &self.path {
            write!(f, "::{}", path)?;
        }

        Ok(())
    }
}

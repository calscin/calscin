//! Definitions for import handling and everything import related

use calsc_utils::hash::HashedString;

use crate::path::PackageLessModulePath;

/// The filter used to check if a path is imported or not
pub struct ImportFilter {
    filter: Vec<HashedString>,
    actual: Vec<HashedString>,
}

impl ImportFilter {
    pub fn new(&self, filter: Vec<HashedString>, actual: Vec<HashedString>) -> Self {
        Self { filter, actual }
    }

    /// Checks if the filter matches on the given path
    pub fn matches(&self, path: &PackageLessModulePath) -> bool {
        if path.0.len() < self.filter.len() {
            return false; // Enforces that the given path is not smaller than the filtered path
        }

        &self.filter == &path.0[0..self.filter.len()]
    }

    /// Replaces the filtered part with the actual part.
    /// This function does not check for filter matching so make sure to do so.
    pub fn replace_with_actual(&self, path: &PackageLessModulePath) -> PackageLessModulePath {
        assert!(path.0.len() >= self.filter.len());

        let rest = &path.0[self.filter.len()..path.0.len()];

        let mut actual = self.actual.clone();

        for elem in rest {
            actual.push(elem.clone());
        }

        PackageLessModulePath(actual)
    }
}

use std::path::PathBuf;

use crate::globalctx::key::GlobalContextKey;

/// Represents a function that will be lazy loaded within the import resolving
pub struct LazyImportQueueElement {
    pub path: PathBuf,
    pub entry: GlobalContextKey,
}

impl LazyImportQueueElement {
    pub fn new(path: PathBuf, entry: GlobalContextKey) -> Self {
        Self { path, entry }
    }
}

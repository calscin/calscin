use std::collections::HashSet;

use calsc_utils::hash::HashedString;

use crate::lazy::LazyLoadedType;

/// A function that is lazily loaded.
#[derive(Debug, Clone)]
pub struct LazyLoadedFunction {
    pub return_type: LazyLoadedType,
    pub arguments: Vec<(HashedString, LazyLoadedType)>,
    pub type_paramers: HashSet<HashedString>,
}

impl LazyLoadedFunction {
    pub fn new(
        return_type: LazyLoadedType,
        arguments: Vec<(HashedString, LazyLoadedType)>,
    ) -> Self {
        Self {
            return_type,
            arguments,
            type_paramers: HashSet::new(),
        }
    }
}

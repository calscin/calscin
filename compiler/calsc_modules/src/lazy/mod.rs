//! Declarations for type lazy loading.
//! This allows for types to be circularly imported just like functions and allows for types to be loaded only in stage 2 instead of stage 1

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_utils::hash::{HashedCounter, HashedString};

use crate::{path::ModulePath, tree::ModuleTree};

pub mod raw;

/// Represents a lazy loaded type. This should normally live before HIR lowering stage 2 as types will be obtained back there.
#[derive(Debug, Clone)]
pub enum LazyLoadedType {
    Base {
        module_path: ModulePath,
        element_name: HashedString,

        size_specifiers: Vec<usize>,
        type_parameters: Vec<LazyLoadedType>,
    },

    TypeParameter {
        name: HashedString,
        param_ind: usize,
    },

    Reference {
        mutable: bool,
        inner: Box<LazyLoadedType>,
    },

    Array {
        size: Option<usize>,
        inner: Box<LazyLoadedType>,
    },

    Void,
}

pub trait LazyLoadedTypeLike {
    /// Gets the dependencies of the lazy loaded type.
    /// Uses a hashed counter in order to find types that include themselves and prevent it.
    fn get_dependencies<S: DiagnosticSource>(
        &self,
        tree: &ModuleTree,
        counter: &mut HashedCounter<ModulePath>,
    ) -> DiagPossible;
}

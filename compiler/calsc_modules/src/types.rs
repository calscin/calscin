//! Declarations for type lazy loading.
//! This allows for types to be circularly imported just like functions and allows for types to be loaded only in stage 2 instead of stage 1

use std::collections::HashMap;

use calsc_utils::hash::HashedString;

use crate::path::ModulePath;

/// Represents a lazy loaded type. This should normally live before HIR lowering stage 2 as types will be obtained back there.
pub enum LazyLoadedType {
    Base {
        module_path: ModulePath,
        element_name: HashedString,

        kind: LazyLoadedRawType,

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

pub enum LazyLoadedRawType {
    LazyLoadedStruct {
        fields: HashMap<HashedString, (LazyLoadedRawType, usize)>,
        field_order: Vec<HashedString>,
    },

    LazyLoadedNormal,
}

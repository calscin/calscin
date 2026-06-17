use std::collections::HashMap;

use calsc_utils::hash::HashedString;

use crate::lazy::LazyLoadedType;

pub enum LazyLoadedRawTypeKind {
    Simple,

    Struct {
        fields: HashMap<HashedString, (LazyLoadedType, usize)>,
        field_order: Vec<HashedString>,
    },
}

pub struct LazyLoadedRawType {
    pub kind: LazyLoadedRawTypeKind,

    pub functions: HashMap<HashedString, (Vec<LazyLoadedType>, LazyLoadedType)>,
    pub type_params: HashMap<HashedString, usize>,
    pub type_params_iter: Vec<HashedString>,
}

impl LazyLoadedRawType {
    pub fn new(kind: LazyLoadedRawTypeKind) -> Self {
        Self {
            kind,
            functions: HashMap::new(),
            type_params: HashMap::new(),
            type_params_iter: vec![],
        }
    }
}

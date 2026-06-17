use std::collections::HashMap;

use calsc_diagnostics::{DiagPossible, DiagnosticSource, diags::errors::build_already_in_scope};
use calsc_utils::hash::HashedString;

use crate::lazy::LazyLoadedType;

#[derive(Debug, Clone)]
pub enum LazyLoadedRawTypeKind {
    Simple,

    Struct {
        fields: HashMap<HashedString, (LazyLoadedType, usize)>,
        field_order: Vec<HashedString>,
    },
}

#[derive(Debug, Clone)]
pub struct LazyLoadedRawType {
    pub kind: LazyLoadedRawTypeKind,

    pub functions: HashMap<HashedString, (Vec<(HashedString, LazyLoadedType)>, LazyLoadedType)>,
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

    pub fn append_function<S: DiagnosticSource>(
        &mut self,
        name: HashedString,
        ret_type: LazyLoadedType,
        arguments: Vec<(HashedString, LazyLoadedType)>,
        source: &S,
    ) -> DiagPossible {
        if self.functions.contains_key(&name) {
            return Err(build_already_in_scope(&name, source).into());
        }

        self.functions.insert(name, (arguments, ret_type));

        Ok(())
    }
}

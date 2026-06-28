use std::collections::HashMap;

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_utils::hash::{HashedCounter, HashedString};

use crate::{
    lazy::{LazyLoadedType, LazyLoadedTypeLike},
    path::ModulePath,
    tree::ModuleTree,
};

#[derive(Debug, Clone)]
pub enum LazyLoadedRawTypeKind {
    Simple,
    TypeParameter,

    Struct {
        fields: HashMap<HashedString, (LazyLoadedType, usize)>,
        field_order: Vec<HashedString>,
    },
}

#[derive(Debug, Clone)]
pub struct LazyLoadedRawType {
    pub kind: LazyLoadedRawTypeKind,
}

impl LazyLoadedRawType {
    pub fn new(kind: LazyLoadedRawTypeKind) -> Self {
        Self { kind }
    }
}

impl LazyLoadedTypeLike for LazyLoadedRawType {
    fn get_dependencies<S: DiagnosticSource>(
        &self,
        tree: &ModuleTree,
        counter: &mut HashedCounter<ModulePath>,
        source: &S,
    ) -> DiagPossible {
        match &self.kind {
            LazyLoadedRawTypeKind::TypeParameter => Ok(()),
            LazyLoadedRawTypeKind::Simple => Ok(()),
            LazyLoadedRawTypeKind::Struct {
                fields,
                field_order: _,
            } => {
                for field in fields {
                    field.1.0.get_dependencies(tree, counter, source)?;
                }

                Ok(())
            }
        }
    }
}

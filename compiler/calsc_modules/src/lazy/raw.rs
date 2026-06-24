use std::collections::HashMap;

use calsc_diagnostics::{DiagPossible, DiagnosticSource, diags::errors::build_already_in_scope};
use calsc_utils::hash::{HashedCounter, HashedString};

use crate::{
    lazy::{LazyLoadedType, LazyLoadedTypeLike},
    path::ModulePath,
    tree::ModuleTree,
};

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
}

impl LazyLoadedRawType {
    pub fn new(kind: LazyLoadedRawTypeKind) -> Self {
        Self {
            kind,
            functions: HashMap::new(),
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

impl LazyLoadedTypeLike for LazyLoadedRawType {
    fn get_dependencies<S: DiagnosticSource>(
        &self,
        tree: &ModuleTree,
        counter: &mut HashedCounter<ModulePath>,
        source: &S,
    ) -> DiagPossible {
        match &self.kind {
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

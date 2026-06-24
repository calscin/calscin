//! Declarations for type lazy loading.
//! This allows for types to be circularly imported just like functions and allows for types to be loaded only in stage 2 instead of stage 1

use calsc_diagnostics::{DiagPossible, DiagnosticSource, diags::errors::build_expected_entry_type};
use calsc_utils::hash::{HashedCounter, HashedString};

use crate::{
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};

pub mod raw;

/// Represents a lazy loaded type. This should normally live before HIR lowering stage 2 as types will be obtained back there.
#[derive(Debug, Clone)]
pub enum LazyLoadedType {
    Base {
        module_path: ModulePath,
        element_name: HashedString,

        size_specifiers: Vec<usize>,
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
        source: &S,
    ) -> DiagPossible;
}

impl LazyLoadedTypeLike for LazyLoadedType {
    fn get_dependencies<S: DiagnosticSource>(
        &self,
        tree: &ModuleTree,
        counter: &mut HashedCounter<ModulePath>,
        source: &S,
    ) -> DiagPossible {
        match self {
            Self::Array { size: _, inner } => inner.get_dependencies(tree, counter, source),
            Self::Reference { mutable: _, inner } => inner.get_dependencies(tree, counter, source),
            Self::Base {
                module_path,
                element_name,
                size_specifiers: _,
            } => {
                let mut path_to_check = module_path.clone();
                path_to_check.path.push(element_name.clone());

                counter.insert(path_to_check.clone());

                //                if counter.get_count(&path_to_check) >= 1 {
                //                    return Err(build_type_infinite_size(&path_to_check, source).into());
                //                }

                // TODO: make a better infinite size protector as this one does not work as it would flag for multiple fields with the same type.

                let r = tree.traverse_to(path_to_check.clone(), source)?;

                if let ModuleTreeEntry::FilledType(ty) = r {
                    ty.get_dependencies(tree, counter, source)?;
                } else {
                    return Err(build_expected_entry_type(
                        &"type".to_string(),
                        &path_to_check,
                        source,
                    )
                    .into());
                }

                Ok(())
            }

            _ => Ok(()),
        }
    }
}

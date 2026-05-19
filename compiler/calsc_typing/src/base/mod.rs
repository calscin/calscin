//! Definitions for base types. They are also named generics inside of the typing system.

use std::collections::HashMap;

use calsc_diagnostics::diags::errors::build_already_in_scope;
use calsc_utils::hash::HashedString;

use crate::{
    FieldHavingType,
    base::kind::BaseTypeKind,
    func::{DeclBlockAffectedType, TypedFunction},
    tree::Type,
};

pub mod kind;
pub mod structs;

#[derive(Clone)]
pub struct BaseType {
    /// The kind of the base type
    pub kind: BaseTypeKind,

    /// The functions of the given type
    pub functions: HashMap<HashedString, TypedFunction>,
}

pub struct BaseTypeInstance {
    /// The actual used type
    pub ty: BaseType,

    /// The size specifiers of the type
    pub size_specifiers: Vec<usize>,

    /// The type parameters of the type
    pub type_parameters: Vec<Type>,
}

impl BaseTypeInstance {
    /// Creates a new [`BaseType`] instance with the given kind and the given type and size specifiers.
    ///
    /// # Panics
    /// This function will panic if the amount ofsize specifiers aren't equal to the amount required.
    ///
    pub fn new(kind: BaseType, size_specifiers: Vec<usize>, type_parameters: Vec<Type>) -> Self {
        if size_specifiers.len() == kind.kind.get_required_size_parameters() {
            Self {
                ty: kind,
                size_specifiers,
                type_parameters,
            }
        } else {
            panic!(
                "Expected {} size parameters but got {} size parameters",
                kind.kind.get_required_size_parameters(),
                size_specifiers.len()
            )
        }
    }
}

impl DeclBlockAffectedType for BaseTypeInstance {
    fn add_function<K: calsc_diagnostics::DiagnosticSource>(
        &mut self,
        name: HashedString,
        func: TypedFunction,
        source: &K,
    ) -> calsc_diagnostics::DiagPossible {
        panic!("Cannot add functions trough instances! Instances are immutable versions of types")
    }

    fn has_function(&self, name: HashedString, signature: crate::func::TypeSignature) -> bool {
        self.ty.has_function(name, signature)
    }
}

impl DeclBlockAffectedType for BaseType {
    fn add_function<K: calsc_diagnostics::DiagnosticSource>(
        &mut self,
        name: HashedString,
        func: TypedFunction,
        source: &K,
    ) -> calsc_diagnostics::DiagPossible {
        if self.functions.contains_key(&name) {
            return Err(build_already_in_scope(&*name, source).into());
        }

        self.functions.insert(name, func);
        Ok(())
    }

    fn has_function(&self, name: HashedString, signature: crate::func::TypeSignature) -> bool {
        self.functions.contains_key(&name)
            && self.functions[&name].arguments == signature.0
            && self.functions[&name].return_type == signature.1
    }
}

impl FieldHavingType for BaseType {
    fn has_field(&self, name: HashedString) -> bool {
        self.kind.has_field(name)
    }

    fn get_field_type(&self, name: HashedString) -> Type {
        self.kind.get_field_type(name)
    }
}

impl FieldHavingType for BaseTypeInstance {
    fn has_field(&self, name: HashedString) -> bool {
        self.ty.has_field(name)
    }

    fn get_field_type(&self, name: HashedString) -> Type {
        self.ty.get_field_type(name)
    }
}

impl PartialEq for BaseType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

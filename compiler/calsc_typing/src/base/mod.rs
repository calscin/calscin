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

pub mod instance;
pub mod kind;
pub mod structs;

#[derive(Clone, Debug)]
/// Represents an actual base type. This is the type that should be registered as a type
pub struct BaseType {
    /// The kind of the base type
    pub kind: BaseTypeKind,

    /// The type parameters of the type
    pub type_params: HashMap<HashedString, usize>,

    /// The functions of the given type
    pub functions: HashMap<HashedString, TypedFunction>,
}

impl BaseType {
    /// Creates a new [`BaseType`] instance with the given kind.
    ///
    pub fn new(kind: BaseTypeKind) -> Self {
        Self {
            kind,
            type_params: HashMap::new(),
            functions: HashMap::new(),
        }
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

impl PartialEq for BaseType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

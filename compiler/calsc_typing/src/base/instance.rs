use calsc_utils::hash::HashedString;

use crate::{
    FieldHavingType,
    base::BaseType,
    func::{DeclBlockAffectedType, TypeSignature, TypedFunction},
    params::resolve_type_parameter_type,
    tree::Type,
};

/// Represents an instance of a [`BaseType`]. Stores the base type, size speicifers and type parameters.
#[derive(PartialEq, Clone, Debug)]
pub struct BaseTypeInstance {
    /// The actual used type
    pub ty: BaseType,

    /// The size specifiers of the type
    pub size_specifiers: Vec<usize>,

    /// The type parameters of the type
    pub type_parameters: Vec<Type>,
}

impl BaseTypeInstance {
    /// Creates a new [`BaseTypeInstance`] instance with the given kind and the given type and size specifiers.
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
        _name: HashedString,
        _func: TypedFunction,
        _source: &K,
    ) -> calsc_diagnostics::DiagPossible {
        panic!("Cannot add functions trough instances! Instances are immutable versions of types")
    }

    fn has_function(&self, name: HashedString) -> bool {
        self.ty.has_function(name)
    }

    fn get_func_signature(&self, name: HashedString) -> TypeSignature {
        let signature = self.ty.get_func_signature(name);

        let mut arguments = vec![];

        for argument in &signature.0 {
            arguments.push(resolve_type_parameter_type(argument.clone(), self)); // Resolves type parameters
        }

        let return_type;

        if signature.1.is_none() {
            return_type = None;
        } else {
            return_type = Some(resolve_type_parameter_type(signature.1.unwrap(), self));
        }

        (arguments, return_type)
    }
}

impl FieldHavingType for BaseTypeInstance {
    fn has_field(&self, name: HashedString) -> bool {
        self.ty.has_field(name)
    }

    fn get_field_type(&self, name: HashedString) -> Type {
        resolve_type_parameter_type(self.ty.get_field_type(name), self) // Resolves type parameters
    }
}

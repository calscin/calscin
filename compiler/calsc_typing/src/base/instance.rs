use calsc_utils::hash::HashedString;

use crate::{
    FieldHavingType, TransmutableType,
    base::BaseType,
    func::{DeclBlockAffectedType, TypeSignature},
    params::resolve_type_parameter_type,
    tree::Type,
};

/// Represents an instance of a [`BaseType`]. Stores the base type, size speicifers and type parameters.
#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
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
    /// This function will panic if the amount of size specifiers or type parameters aren't equal to the amount required.
    ///
    pub fn new(kind: BaseType, size_specifiers: Vec<usize>, type_parameters: Vec<Type>) -> Self {
        if size_specifiers.len() != kind.kind.get_required_size_parameters() {
            panic!(
                "Expected {} size parameters but got {} size parameters",
                kind.kind.get_required_size_parameters(),
                size_specifiers.len()
            );
        }

        if type_parameters.len() != kind.type_params.len() {
            panic!(
                "Expected {} type parameters but got {} type parameters",
                kind.type_params.len(),
                type_parameters.len()
            );
        }

        Self {
            ty: kind,
            size_specifiers,
            type_parameters,
        }
    }
}

impl DeclBlockAffectedType for BaseTypeInstance {
    fn has_function(&self, name: HashedString) -> bool {
        self.ty.has_function(name)
    }

    fn get_func_signature(&self, name: HashedString) -> TypeSignature {
        let signature = self.ty.get_func_signature(name);

        let mut arguments = vec![];

        for argument in &signature.0 {
            arguments.push(resolve_type_parameter_type(argument.clone(), self)); // Resolves type parameters
        }

        let return_type = resolve_type_parameter_type(signature.1, self);

        (arguments, return_type)
    }
}

impl FieldHavingType for BaseTypeInstance {
    fn has_field(&self, name: HashedString) -> bool {
        self.ty.has_field(name)
    }

    fn get_fields(&self) -> Vec<HashedString> {
        self.ty.get_fields()
    }

    fn get_field_index(&self, name: HashedString) -> usize {
        self.ty.get_field_index(name)
    }

    fn get_field_type(&self, name: HashedString) -> Type {
        resolve_type_parameter_type(self.ty.get_field_type(name), self) // Resolves type parameters
    }
}

impl TransmutableType for BaseTypeInstance {
    fn can_transmute(&self, into: Self) -> bool {
        if !self.ty.can_transmute(into.ty) {
            return false;
        }

        if !self.size_specifiers.is_empty() {
            for i in 0..self.size_specifiers.len() {
                if self.size_specifiers[i] > into.size_specifiers[i] {
                    return false;
                }
            }
        }

        self.type_parameters == into.type_parameters
    }

    fn can_transmute_weakly(&self, into: Self) -> bool {
        if !self.ty.can_transmute(into.ty) {
            return false;
        }

        self.type_parameters == into.type_parameters
    }
}

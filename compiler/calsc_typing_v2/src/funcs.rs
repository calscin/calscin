use calsc_utils::hash::HashedString;

use crate::types::TypeKind;

/// Represents a function with a name.
pub struct NamedFunction {
    pub name: HashedString,
    pub return_type: TypeKind,
    pub arguments: Vec<TypeKind>,
}

/// Represents a function without a name
pub struct UnNamedFunction {
    pub return_type: TypeKind,
    pub argument: Vec<TypeKind>,
}

impl Into<UnNamedFunction> for NamedFunction {
    fn into(self) -> UnNamedFunction {
        UnNamedFunction {
            return_type: self.return_type,
            argument: self.arguments,
        }
    }
}

use calsc_utils::hash::HashedString;

use crate::types::TypeKind;

/// Represents a function with a name.
pub struct NamedFunction {
    pub name: HashedString,
    pub return_type: TypeKind,
    pub arguments: Vec<TypeKind>,
}

/// Represents a function without a name
pub struct UnnamedFunction {
    pub return_type: TypeKind,
    pub argument: Vec<TypeKind>,
}

impl Into<UnnamedFunction> for NamedFunction {
    fn into(self) -> UnnamedFunction {
        UnnamedFunction {
            return_type: self.return_type,
            argument: self.arguments,
        }
    }
}

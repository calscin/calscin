use crate::types::TypeKind;

/// Represents a function inside of the type system.
/// Functions within the type system should not hold any name / module path since it is the HIR job to do that.
#[derive(PartialEq, Clone)]
pub struct TypedFunction {
    pub return_type: TypeKind,
    pub argument: Vec<TypeKind>,
}

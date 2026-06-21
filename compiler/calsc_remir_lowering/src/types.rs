use calsc_diagnostics::DiagResult;
use calsc_typing::{
    FieldHavingType,
    base::{instance::BaseTypeInstance, kind::BaseTypeKind},
    tree::Type,
};
use remir::values::ValueType;

pub fn lower_type_base(ty: BaseTypeInstance) -> DiagResult<ValueType> {
    match ty.ty.kind {
        BaseTypeKind::Boolean => Ok(ValueType::Int(false, 1)),
        BaseTypeKind::Char => Ok(ValueType::Int(false, 8)),
        BaseTypeKind::Integer { signed } => Ok(ValueType::Int(signed, ty.size_specifiers[0])),
        BaseTypeKind::Floating { signed: _ } => Ok(ValueType::Float(ty.size_specifiers[0])),
        BaseTypeKind::String => Ok(ValueType::new_any_pointer()),
        BaseTypeKind::Struct(_) => {
            let mut type_fields = vec![];

            for field in ty.get_fields() {
                type_fields.push(Box::new(lower_type(ty.get_field_type(field))?));
            }

            Ok(ValueType::Struct(type_fields))
        }
    }
}

pub fn lower_type(ty: Type) -> DiagResult<ValueType> {
    match ty {
        Type::Base(instance) => lower_type_base(instance),
        Type::Reference { mutable: _, inner } => {
            Ok(ValueType::Pointer(Box::new(lower_type(*inner)?)))
        }

        Type::Pointer { mutable: _, inner } => {
            Ok(ValueType::Reference(Box::new(lower_type(*inner)?)))
        }

        Type::Array { size, inner } => Ok(ValueType::Array(Box::new(lower_type(*inner)?), size)),

        Type::TypeParameter { name, param_ind } => panic!(
            "Invalid type parameter type got into lowering! {} and ind {}",
            name, param_ind
        ),

        Type::Void => Ok(ValueType::Void),
    }
}

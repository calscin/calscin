use calsc_typing::types::TypeKind;

pub fn get_true_field_index(ty: TypeKind, field_ind: usize) -> usize {
    if ty.is_directly_primitive() && ty.as_primitive().ty.is_enum_entry() {
        field_ind + 1 // Account for the marker
    } else {
        field_ind
    }
}

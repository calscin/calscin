use crate::types::TypeKind;

pub enum TypeCell {
    Any,
    Precise(TypeKind),
}

impl TypeCell {
    pub fn matches_type(&self, kind: &TypeKind) -> bool {
        match self {
            Self::Any => true,
            Self::Precise(ty) => ty == kind,
        }
    }
}

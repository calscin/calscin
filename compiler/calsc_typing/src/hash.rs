use std::hash::{DefaultHasher, Hash, Hasher};

use calsc_utils::display_with_to_string;

use crate::{ctx::TypeCtx, types::TypeKind};

pub struct HashedTypeKind {
    pub kind: TypeKind,
    hash: u64,
}

impl HashedTypeKind {
    pub fn new(kind: TypeKind, ctx: &TypeCtx) -> Self {
        let mut state = DefaultHasher::new();

        display_with_to_string(&kind, ctx).hash(&mut state); // TODO: check if this does collisions

        Self {
            kind,
            hash: state.finish(),
        }
    }
}

impl Hash for HashedTypeKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

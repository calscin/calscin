use std::collections::HashMap;

use calsc_modules::path::ModulePath;
use calsc_tree_build::ctx::TreeBuildingCtx;
use calsc_typing::{ctx::TypeCtx, params::TypeParameterId, types::TypeKind};
use calsc_utils::hash::HashedString;

pub struct TreeLowCtx {
    pub build_ctx: TreeBuildingCtx,
    pub type_ctx: TypeCtx,
    pub lowered_map: HashMap<ModulePath, TreeLoweredEntry>,
}

pub enum TreeLoweredEntry {
    Function(
        TypeKind,
        Vec<(TypeKind, HashedString)>,
        Vec<TypeParameterId>,
    ),

    ExternFunc(TypeKind, Vec<(TypeKind, HashedString)>, Option<usize>),

    Struct(Vec<(TypeKind, HashedString)>, Vec<TypeParameterId>),
}

impl TreeLowCtx {
    pub fn new(build_ctx: TreeBuildingCtx) -> Self {
        Self {
            build_ctx,
            type_ctx: TypeCtx::new(),
            lowered_map: HashMap::new(),
        }
    }
}

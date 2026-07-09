use std::{collections::HashMap, fmt::Display};

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_cannot_find_element_no_closest, build_expected_entry_type},
};
use calsc_modules::{path::ModulePath, visibility::Visibility};
use calsc_tree_build::ctx::TreeBuildingCtx;
use calsc_typing::{
    ctx::TypeCtx,
    params::TypeParameterId,
    types::{TypeKind, primitive::PrimitiveType},
};
use calsc_utils::hash::HashedString;

use crate::prelude::apply_lower_prelude;

#[derive(Debug)]
pub struct TreeLowCtx {
    pub build_ctx: TreeBuildingCtx,
    pub type_ctx: TypeCtx,
    pub lowered_map: HashMap<ModulePath, TreeLoweredEntry>,
}

#[derive(Debug)]
pub struct LoweredFunctionContainer(
    pub TypeKind,
    pub Vec<(TypeKind, HashedString)>,
    pub Vec<TypeParameterId>,
    pub Visibility,
);

#[derive(Debug)]
pub struct LoweredExternFuncContainer(
    pub TypeKind,
    pub Vec<(TypeKind, HashedString)>,
    pub Option<usize>,
    pub Visibility,
);

#[derive(Debug)]
pub struct LoweredTypeContainer(pub PrimitiveType, pub Visibility);

#[derive(Debug)]
pub enum TreeLoweredEntry {
    Function(LoweredFunctionContainer),

    ExternFunc(LoweredExternFuncContainer),

    Type(LoweredTypeContainer),
}

impl TreeLowCtx {
    pub fn new(build_ctx: TreeBuildingCtx) -> Self {
        let mut ctx = Self {
            build_ctx,
            type_ctx: TypeCtx::new(),
            lowered_map: HashMap::new(),
        };

        apply_lower_prelude(&mut ctx);

        ctx
    }

    pub fn get_entry<'a, S: DiagnosticSource>(
        &'a self,
        path: &ModulePath,
        source: &S,
    ) -> DiagResult<&'a TreeLoweredEntry> {
        if !self.lowered_map.contains_key(path) {
            return Err(build_cannot_find_element_no_closest(path, source).into());
        }

        Ok(&self.lowered_map[path])
    }
}

impl TreeLoweredEntry {
    pub fn as_type<'a, S: DiagnosticSource>(
        &'a self,
        source: &S,
    ) -> DiagResult<&'a LoweredTypeContainer> {
        match self {
            Self::Type(ty) => Ok(ty),

            _ => return Err(build_expected_entry_type(&"type".to_string(), self, source).into()),
        }
    }
}

impl Display for TreeLoweredEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExternFunc(_) => write!(f, "external function"),
            Self::Function(_) => write!(f, "function"),
            Self::Type(_) => write!(f, "type"),
        }
    }
}

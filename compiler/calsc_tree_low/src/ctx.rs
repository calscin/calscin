use std::{collections::HashMap, fmt::Display};

use calsc_diagnostics::{
    DiagResult, DiagnosticSource,
    diags::errors::{build_cannot_find_element_no_closest, build_expected_entry_type},
};
use calsc_modules::{path::ModulePath, visibility::Visibility};
use calsc_tree_build::ctx::TreeBuildingCtx;
use calsc_typing::{
    TypingInterner,
    ctx::TypeCtx,
    params::TypeParameterId,
    types::{TypeKind, primitive::PrimitiveType},
};
use calsc_utils::hash::HashedString;

use crate::prelude::apply_lower_prelude;

#[derive(Debug)]
pub struct TreeLowCtx<'session> {
    pub type_interner: &'session mut TypingInterner,

    pub data: TreeLowCtxData,
}

#[derive(Debug)]
pub struct TreeLowCtxData {
    pub build_ctx: TreeBuildingCtx,
    pub type_ctx: TypeCtx,
    pub lowered_map: HashMap<ModulePath, TreeLoweredEntry>,
}

#[derive(Debug, Clone)]
pub struct LoweredFunctionContainer(
    pub TypeKind,
    pub Vec<(TypeKind, HashedString)>,
    pub Vec<TypeParameterId>,
    pub Visibility,
);

#[derive(Debug, Clone)]
pub struct LoweredExternFuncContainer(
    pub TypeKind,
    pub Vec<(TypeKind, HashedString)>,
    pub Option<usize>,
    pub Visibility,
);

#[derive(Debug, Clone)]
pub struct LoweredTypeContainer(pub PrimitiveType, pub Visibility);

#[derive(Debug, Clone)]
pub enum TreeLoweredEntry {
    Function(LoweredFunctionContainer),

    ExternFunc(LoweredExternFuncContainer),

    Type(LoweredTypeContainer),
}

impl<'session> TreeLowCtx<'session> {
    pub fn new(build_ctx: TreeBuildingCtx, interner: &'session mut TypingInterner) -> Self {
        let mut ctx = Self {
            type_interner: interner,
            data: TreeLowCtxData::new(build_ctx),
        };

        apply_lower_prelude(&mut ctx);

        ctx
    }

    pub fn get_entry<'a, S: DiagnosticSource>(
        &'a self,
        path: &ModulePath,
        source: &S,
    ) -> DiagResult<&'a TreeLoweredEntry> {
        if !self.data.lowered_map.contains_key(path) {
            return Err(build_cannot_find_element_no_closest(path, source).into());
        }

        Ok(&self.data.lowered_map[path])
    }

    pub fn as_data(self) -> TreeLowCtxData {
        self.data
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

impl TreeLowCtxData {
    pub fn new(build_ctx: TreeBuildingCtx) -> Self {
        Self {
            build_ctx: build_ctx,
            type_ctx: TypeCtx::new(),
            lowered_map: HashMap::new(),
        }
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

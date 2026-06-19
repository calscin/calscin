//! Prelude application related

use calsc_hir::BUILD_CACHE;
use calsc_modules::path::ModulePath;
use calsc_typing::base::{BaseType, kind::BaseTypeKind};

pub fn apply_prelude_to_module_tree_lowering() {
    BUILD_CACHE.with_borrow_mut(|cache| {
        cache.type_storage.map.insert(
            ModulePath::new_module_tree_prelude_path(vec!["bool".into()]),
            BaseType::new(BaseTypeKind::Boolean),
        );

        cache.type_storage.map.insert(
            ModulePath::new_module_tree_prelude_path(vec!["s".into()]),
            BaseType::new(BaseTypeKind::Integer { signed: true }),
        );

        cache.type_storage.map.insert(
            ModulePath::new_module_tree_prelude_path(vec!["u".into()]),
            BaseType::new(BaseTypeKind::Integer { signed: false }),
        );

        cache.type_storage.map.insert(
            ModulePath::new_module_tree_prelude_path(vec!["f".into()]),
            BaseType::new(BaseTypeKind::Floating { signed: true }),
        );

        cache.type_storage.map.insert(
            ModulePath::new_module_tree_prelude_path(vec!["uf".into()]),
            BaseType::new(BaseTypeKind::Floating { signed: false }),
        );

        cache.type_storage.map.insert(
            ModulePath::new_module_tree_prelude_path(vec!["str".into()]),
            BaseType::new(BaseTypeKind::String),
        );

        cache.type_storage.map.insert(
            ModulePath::new_module_tree_prelude_path(vec!["char".into()]),
            BaseType::new(BaseTypeKind::Char),
        );
    });
}

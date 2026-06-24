//! Type lowering from AST -> Lazy loaded

use calsc_ast::{path::ElementPath, types::ASTType};

use calsc_hir::file::HIRFileContext;
use calsc_modules::{lazy::LazyLoadedType, path::ModulePath, tree::ModuleTree};
use calsc_utils::hash::HashedString;

pub type LazyLoadedTypeId = (ModulePath, HashedString);

pub fn lower_stage0_key(
    name: ElementPath,
    hir_file_ctx: &HIRFileContext,
    tree: &ModuleTree,
) -> (ModulePath, HashedString) {
    assert!(!name.members.is_empty());

    let key;

    // If the length is one then it's either a prelude type or a current module type. We thus check if the prelude type exists
    if name.members.len() == 1 {
        let path = ModulePath::new_module_tree_prelude_path(vec![name.members[0].clone()]);

        if tree.contains(path) {
            key = ModulePath::new_module_tree_prelude_path(vec![]);
        } else {
            key = hir_file_ctx.current_module.clone();
        }
    } else {
        key = ModulePath::new(
            name.members[0].clone(),
            name.members[1..name.members.len() - 1].to_vec(),
        )
    }

    let element_name = name.last();

    (key, element_name)
}

pub fn lower_ast_type_base(
    name: ElementPath,
    size_specifiers: Vec<usize>,
    tree: &ModuleTree,
    hir_file_ctx: &HIRFileContext,
) -> LazyLoadedType {
    assert!(!name.members.is_empty());

    let (key, element_name) = lower_stage0_key(name, hir_file_ctx, tree);

    LazyLoadedType::Base {
        module_path: key,
        element_name,
        size_specifiers,
    }
}

pub fn lower_ast_type(
    typ: ASTType,
    tree: &ModuleTree,
    hir_file_ctx: &HIRFileContext,
) -> LazyLoadedType {
    match typ {
        ASTType::Array(size, inner) => LazyLoadedType::Array {
            size,
            inner: Box::new(lower_ast_type(*inner, tree, hir_file_ctx)),
        },

        ASTType::Reference(mutable, inner) => LazyLoadedType::Reference {
            mutable,
            inner: Box::new(lower_ast_type(*inner, tree, hir_file_ctx)),
        },

        ASTType::Generic(path, size_specifier) => {
            let mut size_specifiers = vec![];

            if size_specifier.is_some() {
                size_specifiers.push(size_specifier.unwrap());
            }

            lower_ast_type_base(path, size_specifiers, tree, hir_file_ctx)
        }

        ASTType::Void => LazyLoadedType::Void,
    }
}

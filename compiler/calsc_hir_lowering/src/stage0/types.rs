//! Type lowering from AST -> Lazy loaded

use calsc_ast::{path::ElementPath, types::ASTType};

use calsc_hir::{HIRContext, file::HIRFileContext};
use calsc_modules::{lazy::LazyLoadedType, path::ModulePath, tree::ModuleTree};
use calsc_utils::hash::HashedString;

use crate::stage0::key::lower_stage_0_key;

pub type LazyLoadedTypeId = (ModulePath, HashedString);

pub fn lower_ast_type_base(
    name: ElementPath,
    size_specifiers: Vec<usize>,
    type_parameters: Vec<LazyLoadedType>,
    tree: &ModuleTree,
    hir_file_ctx: &HIRFileContext,
) -> LazyLoadedType {
    assert!(!name.members.is_empty());

    let key;

    // If the length is one then it's either a prelude type or a current module type. We thus check if the prelude type exists
    if name.members.len() == 1 {
        let path = ModulePath::new_prelude_path(vec![name.members[0].clone()]);

        if tree.contains(path) {
            key = ModulePath::new_prelude_path(vec![]);
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

    LazyLoadedType::Base {
        module_path: key,
        element_name,
        size_specifiers,
        type_parameters,
    }
}

pub fn lower_ast_type(typ: ASTType, tree: &ModuleTree) -> LazyLoadedType {
    match typ {
        ASTType::Array(size, inner) => LazyLoadedType::Array {
            size,
            inner: Box::new(lower_ast_type(*inner, tree)),
        },

        ASTType::Reference(mutable, inner) => LazyLoadedType::Reference {
            mutable,
            inner: Box::new(lower_ast_type(*inner, tree)),
        },

        ASTType::Generic(path, size_specifier, type_parameters) => {
            let mut size_specifiers = vec![];
            let mut type_params = vec![];

            for param in type_parameters {
                type_params.push(lower_ast_type(param, tree));
            }

            if size_specifier.is_some() {
                size_specifiers.push(size_specifier.unwrap());
            }

            lower_ast_type_base(path, size_specifiers, type_params)
        }

        ASTType::Void => LazyLoadedType::Void,
    }
}

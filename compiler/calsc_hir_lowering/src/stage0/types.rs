//! Type lowering from AST -> Lazy loaded

use calsc_ast::{path::ElementPath, types::ASTType};

use calsc_modules::lazy::LazyLoadedType;

use crate::stage0::key::lower_stage_0_key;

pub fn lower_ast_type_base(
    name: ElementPath,
    size_specifiers: Vec<usize>,
    type_parameters: Vec<LazyLoadedType>,
) -> LazyLoadedType {
    let key = lower_stage_0_key(name);

    LazyLoadedType::Base {
        module_path: key.0.clone(),
        element_name: key.1,
        size_specifiers,
        type_parameters,
    }
}

pub fn lower_ast_type(typ: ASTType) -> LazyLoadedType {
    match typ {
        ASTType::Array(size, inner) => LazyLoadedType::Array {
            size,
            inner: Box::new(lower_ast_type(*inner)),
        },

        ASTType::Reference(mutable, inner) => LazyLoadedType::Reference {
            mutable,
            inner: Box::new(lower_ast_type(*inner)),
        },

        ASTType::Generic(path, size_specifier, type_parameters) => {
            let mut size_specifiers = vec![];
            let mut type_params = vec![];

            for param in type_parameters {
                type_params.push(lower_ast_type(param));
            }

            if size_specifier.is_some() {
                size_specifiers.push(size_specifier.unwrap());
            }

            lower_ast_type_base(path, size_specifiers, type_params)
        }

        ASTType::Void => LazyLoadedType::Void,
    }
}

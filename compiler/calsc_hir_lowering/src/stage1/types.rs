use std::hint::unreachable_unchecked;

use calsc_ast::{
    nodes::{ASTNode, ASTNodeKind},
    types::ASTType,
};
use calsc_diagnostics::{DiagPossible, DiagResult, DiagnosticSource};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_typing::{
    MutableFieldHavingType,
    base::{
        BaseType, instance::BaseTypeInstance, kind::BaseTypeKind, structs::BaseStructContainer,
    },
    tree::Type,
};
use calsc_utils::hash::HashedString;

pub fn lower_ast_struct_declaration(node: ASTNode) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
        type_params,
        fields,
    } = node.kind.clone()
    {
        let key = GlobalContextKey::new(name.clone());

        let mut base_type = BaseType::new(BaseTypeKind::Struct(BaseStructContainer::new(name)));

        for param in type_params {
            base_type.append_type_parameter(param, &node)?;
        }

        for field in fields {
            base_type.add_field(
                field.1,
                lower_ast_type(field.0, &node, Some(base_type.clone()))?,
                &node,
            )?; // We can clone base_type to pass it to lower_ast_type since the base_type here wont be modified by lower_ast_type
        }

        let _ = HIR_CONTEXT.with_borrow_mut(|f| {
            f.scope
                .append(key, GlobalContextValue::Type(base_type), &node)
        })?;

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

pub fn lower_ast_type<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    inst: Option<BaseType>,
) -> DiagResult<Type> {
    match ty {
        ASTType::Array(size, b) => Ok(Type::Array {
            size,
            inner: Box::new(lower_ast_type(*b, origin, inst)?),
        }),

        ASTType::Reference(mutable, b) => Ok(Type::Reference {
            mutable,
            inner: Box::new(lower_ast_type(*b, origin, inst)?),
        }),

        ASTType::Generic(a, b, c) => {
            let base_type = lower_ast_generic_base(a, origin)?;

            let mut size_specifiers = vec![];

            if b.is_some() {
                size_specifiers.push(b.unwrap());
            }

            let mut type_params = vec![];

            for param in c {
                type_params.push(Type::Base(BaseTypeInstance::new(
                    lower_ast_generic_base(param.into(), origin)?,
                    vec![],
                    vec![],
                )));
            }

            Ok(Type::Base(BaseTypeInstance::new(
                base_type,
                size_specifiers,
                type_params,
            )))
        }
    }
}

pub fn lower_ast_generic_base<K: DiagnosticSource>(
    name: HashedString,
    origin: &K,
) -> DiagResult<BaseType> {
    let key = GlobalContextKey::new(name);

    let base_type = HIR_CONTEXT.with_borrow(|f| f.scope.get_entry(key, origin)?.as_type(origin))?;

    Ok(base_type)
}

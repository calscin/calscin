use std::hint::unreachable_unchecked;

use calsc_ast::{
    nodes::{ASTNode, ASTNodeKind},
    types::ASTType,
};
use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{build_expected_error, build_unexpected_error},
};
use calsc_hir::{
    HIR_CONTEXT,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_typing::{
    MutableFieldHavingType,
    base::{
        BaseType, instance::BaseTypeInstance, kind::BaseTypeKind, structs::BaseStructContainer,
    },
    params::TypeParameterHaving,
    tree::Type,
};
use calsc_utils::hash::HashedString;

use crate::stage1::funcs::lower_ast_function_decl_first_stage;

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

pub fn lower_simple_ast_type<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    inst: Option<BaseType>,
) -> DiagResult<BaseType> {
    if let ASTType::Generic(a, b, c) = ty.clone() {
        if b.is_some() || !c.is_empty() {
            return Err(build_expected_error(
                &"type name",
                &lower_ast_type(ty, origin, inst)?,
                origin,
            )
            .into());
        }

        let ty = lower_ast_generic_base(a, origin)?;

        if ty.is_empty_base() {
            if let Type::Base(instance) = ty {
                return Ok(instance.ty);
            } else {
                unreachable!()
            }
        }
    }

    return Err(
        build_expected_error(&"type name", &lower_ast_type(ty, origin, inst)?, origin).into(),
    );
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
            if inst.is_some() {
                let inst = inst.clone().unwrap();

                if inst.has_type_parameter(a.clone()) && b.is_none() && c.is_empty() {
                    return Ok(inst.get_type_parameter_type(a));
                }
            }

            let mut size_specifiers = vec![];
            let mut type_params = vec![];

            if b.is_some() {
                size_specifiers.push(b.unwrap());
            }

            for param in c {
                type_params.push(lower_ast_type(param, origin, inst.clone())?);
            }

            let ty = lower_ast_generic_base(a, origin)?;

            if !ty.is_empty_base() {
                if !size_specifiers.is_empty() || !type_params.is_empty() {
                    return Err(build_unexpected_error(
                        &"additional type parameters or size specifiers".to_string(),
                        origin,
                    )
                    .into());
                }

                return Ok(ty);
            }

            if let Type::Base(instance) = ty {
                let instance = BaseTypeInstance::new(instance.ty, size_specifiers, type_params);

                return Ok(Type::Base(instance));
            } else {
                unreachable!();
            }
        }
    }
}

pub fn lower_ast_generic_base<K: DiagnosticSource>(
    name: HashedString,
    origin: &K,
) -> DiagResult<Type> {
    let key = GlobalContextKey::new(name);

    let ty = HIR_CONTEXT.with_borrow(|f| f.scope.get_entry(key, origin)?.craft_type(origin))?;

    Ok(ty)
}

pub fn lower_ast_decl_block(node: ASTNode) -> DiagPossible {
    if let ASTNodeKind::StructDeclBlock { target, functions } = node.kind.clone() {
        let target = lower_simple_ast_type(target, &node, None)?;

        for func in functions {
            lower_ast_function_decl_first_stage(ASTNode::clone(&func), Some(target.clone()))?;
        }

        Ok(())
    } else {
        unsafe { unreachable_unchecked() }
    }
}

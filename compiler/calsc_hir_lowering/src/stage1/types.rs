use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
    path::ElementPath,
    types::ASTType,
};
use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource,
    diags::errors::{
        build_compile_time_size, build_expected_simple_type, build_internal_hir_node_leaked,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
};
use calsc_typing::{
    MutableFieldHavingType,
    base::{BaseType, kind::BaseTypeKind, structs::BaseStructContainer},
    params::TypeParameterHaving,
    tree::Type,
};

use crate::{
    convert_visibility, stage1::funcs::lower_ast_function_decl_first_stage,
    stage2::key::lower_ast_key,
};

pub fn lower_ast_struct_declaration(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
        type_params,
        fields,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        let key = GlobalContextKey::new(name.clone()).module_path(file_ctx.current_module.clone());

        let mut base_type = BaseType::new(BaseTypeKind::Struct(BaseStructContainer::new(name)));

        for param in type_params {
            base_type.append_type_parameter(param, &node)?;
        }

        for field in fields {
            base_type.add_field(
                field.1,
                lower_ast_type(field.0, &node, Some(base_type.clone()), file_ctx, ctx)?,
                &node,
            )?; // We can clone base_type to pass it to lower_ast_type since the base_type here wont be modified by lower_ast_type
        }

        ctx.scope
            .append(key, GlobalContextValue::Type(base_type), visibility, &node)?;

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_simple_ast_type<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    _inst: Option<BaseType>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<BaseType> {
    if let ASTType::Generic(a, b, c) = ty.clone() {
        if b.is_some() || !c.is_empty() {
            return Err(build_expected_simple_type(origin).into());
        }

        let ty = lower_ast_generic_base(a, vec![], vec![], origin, file_ctx, ctx)?;

        if ty.is_empty_base() {
            if let Type::Base(instance) = ty {
                return Ok(instance.ty);
            } else {
                unreachable!()
            }
        }
    }

    return Err(build_expected_simple_type(origin).into());
}

pub fn lower_ast_type<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    inst: Option<BaseType>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<Type> {
    lower_ast_type_complex(ty, origin, inst, false, file_ctx, ctx)
}

pub fn lower_ast_type_complex<K: DiagnosticSource>(
    ty: ASTType,
    origin: &K,
    inst: Option<BaseType>,
    allow_compile_time_uncertain_types: bool,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<Type> {
    match ty.clone() {
        ASTType::Array(size, b) => {
            if size.is_none() && !allow_compile_time_uncertain_types {
                return Err(build_compile_time_size(&ty, origin).into());
            }

            Ok(Type::Array {
                size,
                inner: Box::new(lower_ast_type_complex(
                    *b,
                    origin,
                    inst,
                    allow_compile_time_uncertain_types,
                    file_ctx,
                    ctx,
                )?),
            })
        }

        ASTType::Reference(mutable, b) => Ok(Type::Reference {
            mutable,
            inner: Box::new(lower_ast_type_complex(
                *b, origin, inst, true, file_ctx, ctx,
            )?),
        }),

        ASTType::Generic(a, b, c) => {
            if inst.is_some() {
                let inst = inst.clone().unwrap();

                if a.members.len() == 1
                    && inst.has_type_parameter(a.last())
                    && b.is_none()
                    && c.is_empty()
                {
                    return Ok(inst.get_type_parameter_type(a.last()));
                }
            }

            let mut size_specifiers = vec![];
            let mut type_params = vec![];

            if b.is_some() {
                size_specifiers.push(b.unwrap());
            }

            for param in c {
                type_params.push(lower_ast_type_complex(
                    param,
                    origin,
                    inst.clone(),
                    false,
                    file_ctx,
                    ctx,
                )?);
            }

            let ty =
                lower_ast_generic_base(a, size_specifiers, type_params, origin, file_ctx, ctx)?;

            Ok(ty)
        }

        ASTType::Void => Ok(Type::Void),
    }
}

pub fn lower_ast_generic_base<K: DiagnosticSource>(
    name: ElementPath,
    size_specifiers: Vec<usize>,
    type_parameters: Vec<Type>,
    origin: &K,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
) -> DiagResult<Type> {
    let key = lower_ast_key(name, origin, true, file_ctx, ctx)?;

    let ty = ctx
        .scope
        .get_entry(key, &file_ctx.current_module, origin)?
        .craft_type(origin, size_specifiers, type_parameters)?;

    Ok(ty)
}

pub fn lower_ast_decl_block(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclBlock { target, functions } = node.kind.clone() {
        let target = lower_simple_ast_type(target, &node, None, file_ctx, ctx)?;

        for func in functions {
            lower_ast_function_decl_first_stage(
                ast_ctx.nodes.get(&func).clone(),
                Some(target.clone()),
                file_ctx,
                ctx,
            )?;
        }

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

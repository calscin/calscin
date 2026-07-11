use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_modules::path::ModulePath;

use crate::{
    convert_visibility,
    ctx::{LoweredExternFuncContainer, LoweredFunctionContainer, TreeLowCtx, TreeLoweredEntry},
    types::lower_ast_type,
};

pub fn lower_ast_function_declaration(
    node: ASTNode,
    ctx: &mut TreeLowCtx,
    path: ModulePath,
) -> DiagPossible {
    if let ASTNodeKind::FunctionDeclaration {
        name: _,
        arguments,
        return_type,
        body: _,
        visibility,
        type_parameters,
    } = node.kind.clone()
    {
        let group = ctx.data.type_ctx.type_params.start_param_group();

        let visibility = convert_visibility(visibility, &path);
        let mut owned_type_params = vec![];

        for type_parameter in type_parameters {
            let id = ctx
                .data
                .type_ctx
                .type_params
                .append_type_param(type_parameter, &node)?;

            owned_type_params.push(id);
        }

        let return_type = lower_ast_type(&return_type, ctx, &node)?;
        let mut lowered_arugments = vec![];

        for argument in arguments {
            lowered_arugments.push((lower_ast_type(&argument.0, ctx, &node)?, argument.1));
        }

        ctx.data.lowered_map.insert(
            path,
            TreeLoweredEntry::Function(LoweredFunctionContainer(
                return_type,
                lowered_arugments,
                owned_type_params,
                visibility,
            )),
        );

        ctx.data.type_ctx.type_params.end_group(group);
        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_extern_function_declaration(
    node: ASTNode,
    ctx: &mut TreeLowCtx,
    path: ModulePath,
) -> DiagPossible {
    if let ASTNodeKind::ExternFunctionDeclaration {
        name: _,
        arguments,
        return_type,
        triple_dot_position,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, &path);

        let return_type = lower_ast_type(&return_type, ctx, &node)?;
        let mut lowered_arguments = vec![];

        for argument in arguments {
            lowered_arguments.push((lower_ast_type(&argument.0, ctx, &node)?, argument.1));
        }

        ctx.data.lowered_map.insert(
            path,
            TreeLoweredEntry::ExternFunc(LoweredExternFuncContainer(
                return_type,
                lowered_arguments,
                triple_dot_position,
                visibility,
            )),
        );

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

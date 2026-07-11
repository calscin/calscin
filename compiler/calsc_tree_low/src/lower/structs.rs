use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_modules::path::ModulePath;
use calsc_typing::types::{
    primitive::PrimitiveType,
    structs::{NamedField, StructContainer},
};

use crate::{
    convert_visibility,
    ctx::{LoweredTypeContainer, TreeLowCtx, TreeLoweredEntry},
    types::lower_ast_type,
};

pub fn lower_ast_struct_declaration(
    node: ASTNode,
    ctx: &mut TreeLowCtx,
    path: ModulePath,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
        fields,
        visibility,
        type_parameters,
    } = node.kind.clone()
    {
        let group = ctx.data.type_ctx.type_params.start_param_group();

        let mut struct_container = StructContainer::new(name.clone(), path.clone());

        for type_parameter in type_parameters {
            ctx.data
                .type_ctx
                .type_params
                .append_type_param(type_parameter.clone(), &node)?;

            struct_container.type_parameters.push(type_parameter);
        }

        let visibility = convert_visibility(visibility, &path);

        for (field_type, field_name) in fields {
            let field_type = lower_ast_type(&field_type, ctx, &node)?;

            struct_container
                .fields
                .append_named(NamedField(field_name, field_type), &node)?;
        }

        let struct_container = ctx
            .type_interner
            .struct_container_arena
            .append(struct_container);

        let primitive = PrimitiveType::Struct(struct_container);

        ctx.data.lowered_map.insert(
            path,
            TreeLoweredEntry::Type(LoweredTypeContainer(primitive, visibility)),
        );

        ctx.data.type_ctx.type_params.end_group(group);
        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

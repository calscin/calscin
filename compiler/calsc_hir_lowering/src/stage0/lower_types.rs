use std::collections::HashMap;

use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
    types::ASTType,
};
use calsc_diagnostics::{
    DiagPossible, DiagResult,
    diags::errors::{build_expected_simple_type, build_internal_hir_node_leaked},
};
use calsc_hir::{file::HIRFileContext, nodes::HIRNodeKind};
use calsc_modules::{
    lazy::raw::{LazyLoadedRawType, LazyLoadedRawTypeKind},
    tree::{ModuleTree, entry::ModuleTreeEntry},
};

use crate::{
    convert_visibility,
    stage0::{
        func::lower_ast_function_decl_stage_zero, key::lower_stage_0_key, types::lower_ast_type,
    },
};

pub fn lower_ast_type_struct_declaration(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
        type_params,
        fields,
        visibility,
    } = node.kind.clone()
    {
        let visibility = convert_visibility(visibility, file_ctx.current_module.clone());

        if !visibility.can_be_imported() {
            return Ok(());
        }

        let mut lowered_fields = HashMap::new();
        let mut field_order = vec![];

        let mut ind = 0;
        for field in fields {
            lowered_fields.insert(field.1.clone(), (lower_ast_type(field.0), ind));
            field_order.push(field.1);

            ind += 1;
        }

        let kind = LazyLoadedRawTypeKind::Struct {
            fields: lowered_fields,
            field_order,
        };

        let mut raw_type = LazyLoadedRawType::new(kind);

        let mut ind = 0;
        for param in type_params {
            raw_type.type_params.insert(param.clone(), ind);
            raw_type.type_params_iter.push(param);

            ind += 1;
        }

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.path.push(name);

        tree.traverse_to_append(path_to_append_to, ModuleTreeEntry::Type(raw_type), &node)
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

pub fn lower_ast_type_struct_decl(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
    ast_ctx: &ASTContext,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclBlock { target, functions } = node.kind.clone() {
        if let ASTType::Generic(path, b, c) = target {
            if b.is_some() || !c.is_empty() {
                return Err(build_expected_simple_type(&node).into());
            }

            let path = lower_stage_0_key(path);

            for func in functions {
                let func = ast_ctx.nodes.get(&func).clone();

                lower_ast_function_decl_stage_zero(func, Some(path.clone()), file_ctx, tree)?;
            }

            Ok(())
        } else {
            return Err(build_expected_simple_type(&node).into());
        }
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

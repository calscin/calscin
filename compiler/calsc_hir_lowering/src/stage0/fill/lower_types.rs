use std::collections::HashMap;

use calsc_ast::nodes::{ASTNode, ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_hir::{BUILD_CACHE, file::HIRFileContext};
use calsc_modules::{
    lazy::raw::{LazyLoadedRawType, LazyLoadedRawTypeKind},
    tree::{ModuleTree, entry::ModuleTreeEntry},
};

use crate::{convert_visibility, stage0::fill::types::lower_ast_type};

pub fn lower_ast_type_struct_declaration(
    node: ASTNode,
    file_ctx: &mut HIRFileContext,
    tree: &mut ModuleTree,
) -> DiagPossible {
    if let ASTNodeKind::StructDeclaration {
        name,
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
            lowered_fields.insert(
                field.1.clone(),
                (lower_ast_type(field.0, tree, file_ctx), ind),
            );
            field_order.push(field.1);

            ind += 1;
        }

        let kind = LazyLoadedRawTypeKind::Struct {
            fields: lowered_fields,
            field_order,
        };

        let raw_type = LazyLoadedRawType::new(kind);

        let mut path_to_append_to = file_ctx.current_module.clone();
        path_to_append_to.path.push(name);

        // Append the node to the related nodes inside of the build cache
        {
            BUILD_CACHE.with_borrow_mut(|cache| {
                cache.append_related_node(path_to_append_to.clone(), node.clone());
            })
        }

        tree.traverse_to_append(
            path_to_append_to,
            ModuleTreeEntry::FilledType(raw_type),
            &node,
        )
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

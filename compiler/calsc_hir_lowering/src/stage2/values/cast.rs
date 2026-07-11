use calsc_ast::{
    ASTContext,
    nodes::{ASTNode, ASTNodeKind},
};
use calsc_diagnostics::{
    DiagResult,
    diags::{
        errors::{build_internal_hir_node_leaked, build_type_cast_failed},
        warnings::build_useless_cast,
    },
};
use calsc_hir::{
    HIRContext,
    file::HIRFileContext,
    globalctx::key::GlobalContextKey,
    nodes::{HIRNode, HIRNodeKind},
};

use calsc_typing::into::{TypeCasting, TypeTransmutation};
use calsc_utils::{alloc::arena::ArenaHandle, display_with_to_string};

use crate::stage2::{types::lower_ast_type, values::lower_ast_value};

pub fn lower_ast_cast(
    node: ASTNode,
    local_ctx: Option<GlobalContextKey>,
    file_ctx: &mut HIRFileContext,
    ctx: &mut HIRContext,
    ast_ctx: &ASTContext,
) -> DiagResult<ArenaHandle> {
    if let ASTNodeKind::IntoCast { val, into } = node.kind.clone() {
        let val = lower_ast_value(
            ast_ctx.nodes.get(&val).clone(),
            local_ctx.clone(),
            file_ctx,
            ctx,
            ast_ctx,
        )?;

        let val_ref = ctx.nodes.get(&val).clone();

        let val_type = val_ref.get_type(local_ctx.clone(), ctx, Some(file_ctx))?;

        let into = lower_ast_type(&into, &node, ctx)?;

        if !val_type.can_cast(&into, &ctx.session.type_interner) {
            return Err(build_type_cast_failed(
                &display_with_to_string(&val_type, &ctx.session.type_interner),
                &display_with_to_string(&into, &ctx.session.type_interner),
                &node,
            )
            .into());
        }

        if val_type.can_transmute(&into, &ctx.session.type_interner) {
            build_useless_cast(
                &display_with_to_string(&val_type, &ctx.session.type_interner),
                &display_with_to_string(&into, &ctx.session.type_interner),
                &node,
            );
        }

        let node = HIRNode::new(
            HIRNodeKind::CastNode {
                original: val,
                into,
                explicit_cast: true,
            },
            node.start.clone(),
            node.end.clone(),
        );

        Ok(node.push(ctx))
    } else {
        return Err(build_internal_hir_node_leaked(&node, &node).into());
    }
}

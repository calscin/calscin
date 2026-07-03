use calsc_ast::{ASTContext, nodes::ASTNodeKind};
use calsc_diagnostics::{DiagPossible, diags::errors::build_internal_hir_node_leaked};
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{ctx::TreeBuildingCtx, walk::walk_through_node};

pub fn walk_through_module(
    handle: &ArenaHandle,
    ast: &ASTContext,
    ctx: &mut TreeBuildingCtx,
) -> DiagPossible {
    let node_ref = ast.nodes.get(handle);

    if let ASTNodeKind::Module {
        name,
        is_bodied,
        body,
    } = &node_ref.kind
    {
        if !is_bodied {
            return Ok(()); // Doesn't handle file walking here
        }

        ctx.current_path.append_single_bit(name.clone());

        println!("- Scanning module {}", ctx.current_path);

        ctx.tree.append_module(
            &ctx.current_path.clone(),
            ctx.current_file.clone(),
            &mut ctx.arena,
            node_ref,
        )?;

        for node in body {
            walk_through_node(node, ast, ctx)?;
        }

        println!("- End of module {}", ctx.current_path);

        ctx.current_path.path.pop();

        Ok(())
    } else {
        return Err(build_internal_hir_node_leaked(node_ref, node_ref).into());
    }
}

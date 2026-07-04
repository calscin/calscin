use std::path::PathBuf;

use calsc_diagnostics::DiagPossible;
use calsc_modules::treev2::entry::TreeEntryKind;
use calsc_utils::alloc::arena::ArenaHandle;

use crate::{
    ctx::TreeBuildingCtx,
    walk_second_pass::nodes::{walk_second_pass_import, walk_second_pass_node},
};

pub mod nodes;

pub fn walk_second_pass(path: &PathBuf, ctx: &mut TreeBuildingCtx) -> DiagPossible {
    // Set back the path to the base. We do this since the hashmap randomizes the order so we cannot use the same strategy as the first walk
    ctx.current_path = ctx.module_path_base[path].clone();

    println!("% Second pass on {}", ctx.current_path);

    // Resolve imports first
    if ctx.import_nodes.contains_key(&ctx.current_path) {
        for import_node in ctx.import_nodes[&ctx.current_path].clone() {
            walk_second_pass_import(&import_node, &ctx.current_file.clone(), ctx)?;
        }
    } else {
        println!("Doesn't contain! {}", ctx.current_path);
    }

    for (_, entry) in ctx.tree.children.clone() {
        walk_second_pass_entry(entry, ctx)?;
    }

    // Remove the appended module name
    ctx.current_path.path.pop();

    Ok(())
}

pub fn walk_second_pass_entry(entry: ArenaHandle, ctx: &mut TreeBuildingCtx) -> DiagPossible {
    let entry = ctx.arena.get(&entry);

    ctx.current_path = entry.self_path.clone();

    if let TreeEntryKind::Module(module) = &entry.kind {
        for (_, child) in &module.children.clone() {
            walk_second_pass_entry(child.clone(), ctx)?;
        }

        Ok(())
    } else {
        if !entry.has_related_nodes() {
            return Ok(()); // Skip entries without related nodes
        }

        let (path, related_nodes) = ctx.related_nodes[&ctx.current_path].clone();
        for entry in related_nodes {
            walk_second_pass_node(&entry, &path, ctx)?;
        }

        Ok(())
    }
}

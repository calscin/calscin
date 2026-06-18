use calsc_ast::nodes::ASTNode;
use calsc_diagnostics::{DiagPossible, DiagnosticSource, diags::errors::build_expected_entry_type};
use calsc_hir::BUILD_CACHE;
use calsc_modules::{
    lazy::LazyLoadedTypeLike,
    path::ModulePath,
    tree::{ModuleTree, entry::ModuleTreeEntry},
};
use calsc_utils::hash::HashedCounter;

pub fn lower_type_from_tree<S: DiagnosticSource>(
    path: ModulePath,
    tree: &ModuleTree,
    source: &S,
) -> DiagPossible {
    let already_built = BUILD_CACHE.with_borrow(|cache| cache.type_storage.map.contains_key(&path));

    if already_built {
        return Ok(()); // No need to do anything else if its already built.
    }

    let r = tree.traverse_to(path.clone(), source)?;

    if let ModuleTreeEntry::Type(ty) = r {
        let mut dependencies = HashedCounter::new();

        ty.get_dependencies(tree, &mut dependencies, source)?;

        // Lower each dependency first so that the type can be safely resolved
        for dependency in &dependencies.map {
            lower_type_from_tree(dependency.0.clone(), tree, source)?;
        }

        // We then lower each part of the type by lowering each related AST node.
        let related_nodes = BUILD_CACHE.with_borrow(|cache| cache.nodes_to_entries[&path].clone());

        for node in related_nodes {
            lower_type_node(path.clone(), tree, node, source)?;
        }
    } else {
        return Err(build_expected_entry_type(&"type".to_string(), &path, source).into());
    }

    Ok(())
}

pub fn lower_type_node<S: DiagnosticSource>(
    path: ModulePath,
    tree: &ModuleTree,
    node: ASTNode,
    source: &S,
) -> DiagPossible {
    todo!()
}

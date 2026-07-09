use std::collections::HashSet;

use calsc_diagnostics::{
    DiagPossible, DiagResult, DiagnosticSource, diags::errors::build_type_infinite_size,
};
use calsc_modules::{path::ModulePath, visibility::Visibility};

use crate::ctx::TreeLowCtx;

pub mod ctx;
pub mod lower;
pub mod prelude;
pub mod types;

pub fn convert_visibility(
    visibility: Option<calsc_ast::visibility::Visibility>,
    module_path: &ModulePath,
) -> Visibility {
    let visibility = visibility.unwrap_or(calsc_ast::visibility::Visibility::Protected);
    let parent = module_path.everything_but_last();

    match visibility {
        calsc_ast::visibility::Visibility::Public => Visibility::Public,
        calsc_ast::visibility::Visibility::Protected => Visibility::Protected(parent),
        calsc_ast::visibility::Visibility::Private => Visibility::Private(parent),
    }
}

pub fn get_dependencies_of_entry<S: DiagnosticSource>(
    ctx: &TreeLowCtx,
    path: &ModulePath,
    source: &S,
) -> DiagResult<HashSet<ModulePath>> {
    let mut set = HashSet::new();

    get_dependencies_inner(ctx, path, path, &mut set, source)?;

    Ok(set)
}

fn get_dependencies_inner<S: DiagnosticSource>(
    ctx: &TreeLowCtx,
    master: &ModulePath,
    path: &ModulePath,
    set: &mut HashSet<ModulePath>,
    source: &S,
) -> DiagPossible {
    let entry = ctx
        .build_ctx
        .tree
        .get_entry(path, &ctx.build_ctx.arena, source)?;

    for dep in entry.typing_dependencies.clone() {
        if &dep == master {
            return Err(build_type_infinite_size(&dep, source).into());
        }

        get_dependencies_inner(ctx, master, &dep, set, source)?;

        set.insert(dep);
    }

    for dep in entry.semantic_dependencies.clone() {
        get_dependencies_inner(ctx, master, &dep, set, source)?;

        set.insert(dep);
    }

    Ok(())
}

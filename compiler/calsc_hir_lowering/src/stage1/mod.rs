use std::path::PathBuf;

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::{
    HIRContext,
    funcs::HIRFunction,
    globalctx::{key::GlobalContextKey, vals::GlobalContextValue},
    localctx::LocalContext,
};
use calsc_modules::path::ModulePath;
use calsc_tree_low::ctx::TreeLoweredEntry;

pub fn import_everything_inside_module<S: DiagnosticSource>(
    mod_path: ModulePath,
    path: &PathBuf,
    hir: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    for entry in hir
        .state
        .get()
        .get_tree_lowered()
        .build_ctx
        .tree
        .collect_entries(
            &mod_path,
            &hir.state.get().get_tree_lowered().build_ctx.arena,
            path,
            origin,
        )?
    {
        if hir
            .state
            .get()
            .get_tree_lowered()
            .lowered_map
            .contains_key(&entry)
        {
            import_entry_into_hir(
                hir.state.get().get_tree_lowered().lowered_map[&entry].clone(),
                entry,
                hir,
                origin,
            )?;
        }
    }

    Ok(())
}

pub fn import_entry_into_hir<S: DiagnosticSource>(
    entry: TreeLoweredEntry,
    path: ModulePath,
    hir: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    let name = path.last();
    let mut key = GlobalContextKey::new(name.clone()).module_path(path.everything_but_last());

    match entry {
        TreeLoweredEntry::Type(ty) => {
            let _ = hir
                .scope
                .append(key, GlobalContextValue::Type(ty.0), ty.1, origin)?;
        }

        TreeLoweredEntry::Function(container) => {
            let is_main_function = name == "main".into() && path.path.len() == 1;

            if is_main_function {
                key = GlobalContextKey::new("main".into());
            }

            let mut arguments = vec![];

            for (ty, name) in container.1 {
                arguments.push((name, ty));
            }

            let mut func = HIRFunction::new_stage_1(
                key.clone(),
                LocalContext::new(
                    name.clone(),
                    key.clone(),
                    container.0.clone(),
                    is_main_function,
                ),
                container.0,
                arguments,
                is_main_function,
            );

            func.type_parameters = container.2;

            let _ =
                hir.scope
                    .append(key, GlobalContextValue::Function(func), container.3, origin)?;
        }

        TreeLoweredEntry::ExternFunc(container) => {
            let is_main_function = name == "main".into() && path.path.len() == 1;

            let mut arguments = vec![];

            for (ty, name) in container.1 {
                arguments.push((name, ty));
            }

            let func = HIRFunction::new_extern(
                key.clone(),
                container.0,
                arguments,
                container.2,
                is_main_function,
            );

            let _ =
                hir.scope
                    .append(key, GlobalContextValue::Function(func), container.3, origin)?;
        }
    };

    Ok(())
}

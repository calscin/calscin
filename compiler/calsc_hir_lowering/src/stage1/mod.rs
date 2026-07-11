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

use crate::stage1::imports::handle_imports_everything;

pub mod imports;

pub fn import_everything_inside_module<S: DiagnosticSource>(
    mod_path: ModulePath,
    path: &PathBuf,
    hir: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    handle_imports_everything(mod_path.clone(), path, hir, origin)?;

    let tree_lowered = hir.session.get_tree_lowered();
    let build_ctx = &tree_lowered.build_ctx;

    for entry in build_ctx.tree.collect_entries(
        &mod_path,
        &hir.session.get_tree_lowered().build_ctx.arena,
        path,
        origin,
    )? {
        if hir
            .session
            .get_tree_lowered()
            .lowered_map
            .contains_key(&entry)
        {
            import_entry_into_hir(
                hir.session.get_tree_lowered().lowered_map[&entry].clone(),
                entry.clone(),
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
    actual_path: ModulePath,
    hir: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    let name = path.last();
    let fake_key = GlobalContextKey::new(name.clone()).module_path(path.everything_but_last());
    let mut real_key = GlobalContextKey::new(actual_path.last().clone())
        .module_path(actual_path.everything_but_last());

    match entry {
        TreeLoweredEntry::Type(ty) => {
            let _ = hir.scope.append(
                real_key.clone(),
                GlobalContextValue::Type(ty.0),
                ty.1.clone(),
                origin,
            )?;

            if &fake_key != &real_key {
                hir.scope.append(
                    fake_key,
                    GlobalContextValue::AnotherReference(real_key.clone()),
                    ty.1,
                    origin,
                )?;
            }
        }

        TreeLoweredEntry::Function(container) => {
            let is_main_function = name == "main".into() && path.path.len() == 1;

            if is_main_function {
                real_key = GlobalContextKey::new("main".into());
            }

            let mut arguments = vec![];

            for (ty, name) in container.1 {
                arguments.push((name, ty));
            }

            let mut func = HIRFunction::new_stage_1(
                real_key.clone(),
                LocalContext::new(
                    name.clone(),
                    real_key.clone(),
                    container.0.clone(),
                    is_main_function,
                ),
                container.0,
                arguments,
                is_main_function,
            );

            func.type_parameters = container.2;

            hir.scope.append(
                real_key.clone(),
                GlobalContextValue::Function(func),
                container.3.clone(),
                origin,
            )?;

            if &fake_key != &real_key && !is_main_function {
                hir.scope.append(
                    fake_key.clone(),
                    GlobalContextValue::AnotherReference(real_key.clone()),
                    container.3,
                    origin,
                )?;
            }
        }

        TreeLoweredEntry::ExternFunc(container) => {
            let is_main_function = name == "main".into() && path.path.len() == 1;

            let mut arguments = vec![];

            for (ty, name) in container.1 {
                arguments.push((name, ty));
            }

            let func = HIRFunction::new_extern(
                real_key.clone(),
                container.0,
                arguments,
                container.2,
                is_main_function,
            );

            hir.scope.append(
                real_key.clone(),
                GlobalContextValue::Function(func),
                container.3.clone(),
                origin,
            )?;

            if &fake_key != &real_key && !is_main_function {
                hir.scope.append(
                    fake_key,
                    GlobalContextValue::AnotherReference(real_key),
                    container.3,
                    origin,
                )?;
            }
        }
    };

    Ok(())
}

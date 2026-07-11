use std::path::PathBuf;

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_hir::HIRContext;
use calsc_modules::path::{ModulePath, PackageLessModulePath};

use crate::stage1::import_entry_into_hir;

pub fn handle_imports_everything<S: DiagnosticSource>(
    mod_path: ModulePath,
    path: &PathBuf,
    hir: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    for path in hir
        .session
        .get_tree_lowered()
        .build_ctx
        .tree
        .collect_modules(
            &mod_path,
            &hir.session.get_tree_lowered().build_ctx.arena,
            path,
            origin,
        )?
    {
        handle_imports(path, hir, origin)?;
    }

    Ok(())
}

pub fn handle_imports<S: DiagnosticSource>(
    mod_path: ModulePath,
    hir: &mut HIRContext,
    origin: &S,
) -> DiagPossible {
    let entry = hir
        .session
        .get_tree_lowered()
        .build_ctx
        .tree
        .get_entry(
            &mod_path,
            &hir.session.get_tree_lowered().build_ctx.arena,
            origin,
        )?
        .kind
        .as_module(origin)?;

    for import in entry.imports.clone() {
        let mut import_to = mod_path.clone();
        import_to.append_packageless(PackageLessModulePath(import.filter.clone()));

        let import_from = ModulePath::new(import.actual[0].clone(), import.actual[1..].to_vec());

        let entry = hir.session.get_tree_lowered().lowered_map[&import_from].clone();

        import_entry_into_hir(entry, import_to, import_from, hir, origin)?;
    }

    Ok(())
}

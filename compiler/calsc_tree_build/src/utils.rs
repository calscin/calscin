use std::{ffi::OsStr, path::PathBuf};

use calsc_ast::path::ElementPath;
use calsc_diagnostics::{DiagResult, DiagnosticSource};
use calsc_modules::{
    path::{ModulePath, PackageLessModulePath},
    treev2::entry::TreeEntryKind,
};
use calsc_utils::hash::HashedString;

use crate::ctx::TreeBuildingCtx;

pub(crate) fn resolve_path<S: DiagnosticSource>(
    mut path: ElementPath,
    ctx: &TreeBuildingCtx,
    source: &S,
) -> DiagResult<ModulePath> {
    if path.members.len() == 1 {
        // Check for prelude types

        let prelude_path = ModulePath::new_prelude_path(path.members.clone());

        if ctx.tree.has_entry(&prelude_path, &ctx.arena) {
            return Ok(prelude_path);
        }
    }

    if ctx.current_path.get_size() > 1 {
        let parent_module = ctx.current_path.everything_but_last();
        let parent_module = ctx.tree.get_entry(&parent_module, &ctx.arena, source)?;

        // Handle imports
        if let TreeEntryKind::Module(module) = &parent_module.kind {
            let key = PackageLessModulePath(path.members.clone());

            if module.imports.contains_key(&key) {
                return Ok(module.imports[&key].clone());
            }
        }
    }

    let mut curr_path = ctx.current_path.clone();
    curr_path.path.append(&mut path.members);

    Ok(curr_path)
}

pub(crate) fn get_module_name_from_file(file: &PathBuf) -> HashedString {
    let file = if file.file_name() == Some(OsStr::new("module.cal")) {
        &file.parent().expect("not relative path").to_path_buf()
    } else {
        file
    };

    let file = file.with_extension("");

    if file.file_name().is_none() {
        return "".into(); // Handles when the main file is named module.cal
    }

    file.file_name()
        .unwrap()
        .to_str()
        .expect("The file name is not valid unicode")
        .to_string()
        .into()
}

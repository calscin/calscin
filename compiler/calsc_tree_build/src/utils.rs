use std::{ffi::OsStr, path::PathBuf};

use calsc_ast::path::ElementPath;
use calsc_diagnostics::{
    DiagResult, DiagnosticSource, diags::errors::build_cannot_find_element_no_closest,
};
use calsc_modules::{
    path::{ModulePath, PackageLessModulePath},
    treev2::{entry::TreeEntryKind, module::TreeModule, traverse::TraverseTree},
};
use calsc_utils::hash::HashedString;

use crate::ctx::TreeBuildingCtx;

pub(crate) fn matches_any_import(module: &TreeModule, path: &PackageLessModulePath) -> bool {
    for filter in &module.imports {
        if filter.matches(path) {
            return true;
        }
    }

    false
}

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

            for filter in &module.imports {
                if filter.matches(&key) {
                    let key = filter.replace_with_actual(&key);

                    let path = ModulePath::new(key.0[0].clone(), key.0[1..key.0.len()].to_vec());

                    if !ctx.tree.has_entry(&path, &ctx.arena) {
                        return Err(build_cannot_find_element_no_closest(&path, source).into());
                    }

                    return Ok(path);
                }
            }
        }
    }

    let mut curr_path = ctx.current_path.clone();
    curr_path.path.append(&mut path.members);

    println!("{:#?}", curr_path);

    if !ctx.tree.has_entry(&curr_path, &ctx.arena) {
        return Err(build_cannot_find_element_no_closest(&path, source).into());
    }

    Ok(curr_path)
}

pub(crate) fn resolve_import_path(
    path: ElementPath,
    current_path: ModulePath,
    ctx: &TreeBuildingCtx,
) -> ModulePath {
    // First we check if the path is relative by simply checking if the package exists
    let relative = !ctx.tree.has(&path.members[0]);

    if relative {
        let mut module_path = current_path;

        for bit in path.members {
            module_path.append_single_bit(bit);
        }

        module_path
    } else {
        ModulePath {
            package: path.members[0].clone(),
            path: path.members[1..path.members.len()].to_vec(),
        }
    }
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

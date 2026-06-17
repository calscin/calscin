use std::{fs, path::PathBuf};

use calsc_diagnostics::{DiagPossible, DiagnosticSource};
use calsc_modules::{path::ModulePath, tree::ModuleTree};

use crate::modules::module_tree_append_file;

pub fn seek_module_tree_folder<S: DiagnosticSource>(
    path: PathBuf,
    module_path: ModulePath,
    tree: &mut ModuleTree,
    source: &S,
) -> DiagPossible {
    assert!(path.is_dir());

    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();

        if path.extension().is_some() && path.extension().unwrap() == "cal" {
            seek_module_file(path, module_path.clone(), tree, source)?;
        }
    }

    Ok(())
}

pub fn seek_module_file<S: DiagnosticSource>(
    path: PathBuf,
    mut module_path: ModulePath,
    tree: &mut ModuleTree,
    source: &S,
) -> DiagPossible {
    let module_name = path
        .with_extension("")
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // We check for "module" for module.cal files
    if module_name != "module" {
        module_path.path.push(module_name.into());
    }

    module_tree_append_file(path, module_path, tree, source)?;

    Ok(())
}

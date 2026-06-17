use std::{fs, path::PathBuf};

use calsc_diagnostics::DiagPossible;
use calsc_modules::{path::ModulePath, tree::ModuleTree};

use crate::modules::module_tree_append_file;

/// Seeks everyt file inside of the folder and append their contents inside of the module tree inside of the calculated path.
/// Warning: This function creates its own module path based on the given one and the file name.
/// Warning: This function is recursive
pub fn seek_module_tree_folder(
    path: PathBuf,
    module_path: ModulePath,
    tree: &mut ModuleTree,
) -> DiagPossible {
    assert!(path.is_dir(), "file {:#?} is not a folder", path);

    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();

        if path.is_dir() {
            let mut module_path = module_path.clone();

            module_path.path.push(
                path.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into(),
            );

            seek_module_tree_folder(path, module_path, tree)?;
        } else if path.extension().is_some() && path.extension().unwrap() == "cal" {
            seek_module_file(path, module_path.clone(), tree)?;
        }
    }

    Ok(())
}

/// Seeks the given file and append it's content inside of the module tree inside of the calculated module path.
/// Warning: This function creates its own module path based on the given one and the file name.
pub fn seek_module_file(
    path: PathBuf,
    mut module_path: ModulePath,
    tree: &mut ModuleTree,
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

    module_tree_append_file(path, module_path, tree)?;

    Ok(())
}

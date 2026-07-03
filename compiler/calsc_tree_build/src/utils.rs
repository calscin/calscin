use std::{ffi::OsStr, path::PathBuf};

use calsc_utils::hash::HashedString;

pub(crate) fn get_module_name_from_file(file: &PathBuf) -> HashedString {
    let file = if file.file_name() == Some(OsStr::new("module.cal")) {
        &file.parent().unwrap().to_path_buf()
    } else {
        file
    };

    file.with_extension("")
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .into()
}

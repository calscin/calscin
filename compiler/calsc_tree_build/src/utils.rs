use std::{ffi::OsStr, path::PathBuf};

use calsc_utils::hash::HashedString;

pub(crate) fn get_module_name_from_file(file: &PathBuf) -> HashedString {
    let file = if file.file_name() == Some(OsStr::new("module.cal")) {
        &file.parent().expect("not relative path").to_path_buf()
    } else {
        file
    };

    file.with_extension("")
        .file_name()
        .expect("A path not ending by ..")
        .to_str()
        .expect("The file name is not valid unicode")
        .to_string()
        .into()
}

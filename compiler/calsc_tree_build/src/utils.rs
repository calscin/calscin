use std::{ffi::OsStr, path::PathBuf};

use calsc_utils::hash::HashedString;

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

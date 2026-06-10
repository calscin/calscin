use std::{fs, path::PathBuf};

use calsc_builder::{build_file, link_files};
use indicatif::{ProgressBar, ProgressStyle};

pub fn build_command(input: Vec<PathBuf>, out: PathBuf, linker: String) {
    let progress_bar = ProgressBar::new(input.len() as u64);
    let mut object_files = vec![];

    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.green/dark_green}] \
		 	{pos:>3}/{len:3} {msg}",
        )
        .unwrap()
        .progress_chars("=>"),
    );

    let mut ind = 0;
    for path in input {
        let out_object_file = path.with_extension("o");

        build_file(path, out_object_file.clone());

        object_files.push(out_object_file);
    }

    progress_bar.finish_with_message("Linking");

    link_files(object_files, out, linker);

    progress_bar.finish_with_message("Cleaning up");

    for object in object_files {
        fs::read(object);
    }
}

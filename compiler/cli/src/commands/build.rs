use std::{fs, path::PathBuf};

use calsc_builder::{build_file, link_files};
use indicatif::{ProgressBar, ProgressStyle};

pub fn sanitize_path(path: PathBuf) -> PathBuf {
    let s = path.to_string_lossy();

    let trimmed = s.trim_start_matches(' ');

    PathBuf::from(trimmed)
}

pub fn build_command(input: Vec<PathBuf>, out: PathBuf, linker: String, use_pie: bool) {
    let progress_bar = ProgressBar::new(input.len() as u64);
    let mut object_files = vec![];

    let out = sanitize_path(out);
    let input: Vec<PathBuf> = input.iter().map(|f| sanitize_path(f.clone())).collect();

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
        progress_bar.set_position(ind);

        let out_object_file = path.with_extension("o");

        build_file(path, out_object_file.clone(), use_pie);

        object_files.push(out_object_file);

        ind += 1;
    }

    progress_bar.finish_with_message("Linking");

    link_files(object_files.clone(), out, linker);

    progress_bar.finish_with_message("Cleaning up");

    for object in object_files {
        fs::remove_file(object).unwrap()
    }
}

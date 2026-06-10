use std::path::PathBuf;

use calsc_builder::check_file;
use indicatif::{ProgressBar, ProgressStyle};

use crate::commands::build::sanitize_path;

pub fn check_command(input: Vec<PathBuf>, only_ast: bool) {
    let input: Vec<PathBuf> = input.iter().map(|f| sanitize_path(f.clone())).collect();

    let progress_bar = ProgressBar::new(input.len() as u64);

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

        check_file(path, only_ast);

        ind += 1;
    }

    progress_bar.finish_with_message("Finished checking")
}

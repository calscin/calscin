use std::path::PathBuf;

use calsc_builder::compile_remir_file;
use indicatif::{ProgressBar, ProgressStyle};

use crate::commands::build::sanitize_path;

pub fn remir_command(input: Vec<PathBuf>) {
    let progress_bar = ProgressBar::new(input.len() as u64);

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

        let out = path.with_extension("remir");

        compile_remir_file(path, out);

        ind += 1;
    }

    progress_bar.finish_with_message("Finished");
}

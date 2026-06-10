use std::path::PathBuf;

use indicatif::{ProgressBar, ProgressStyle};

pub fn build_command(input: Vec<PathBuf>, out: PathBuf, linker: String) {
    let progress_bar = ProgressBar::new(input.len() as u64);

    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.green/dark_green}] \
		 	{pos:>3}/{len:3} {msg}",
        )
        .unwrap()
        .progress_chars("=>"),
    );
}

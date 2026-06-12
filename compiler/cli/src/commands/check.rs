use std::path::PathBuf;

use calsc_builder::{build, setup_build_state};
use calsc_state::build::BuildTargetMode;

use crate::commands::build::sanitize_path;

pub fn check_command(input: Vec<PathBuf>, _only_ast: bool) {
    let input: Vec<PathBuf> = input.iter().map(|f| sanitize_path(f.clone())).collect();
    let sample_out_fake = PathBuf::from(".");

    setup_build_state(
        sample_out_fake,
        BuildTargetMode::Check,
        input,
        "".to_string(),
        false,
    );

    build();
}

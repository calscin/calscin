use std::path::PathBuf;

use calsc_builder::{build, setup_build_state};
use calsc_state::build::BuildTargetMode;

use crate::commands::build::sanitize_path;

pub fn check_command(input: PathBuf, _only_ast: bool) {
    let input: PathBuf = sanitize_path(input);
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

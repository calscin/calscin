use std::path::PathBuf;

use calsc_builder::{build, setup_build_state};
use calsc_state::build::BuildTargetMode;

use crate::commands::build::sanitize_path;

pub fn ir_command(input: Vec<PathBuf>, remir: bool) {
    let input: Vec<PathBuf> = input.iter().map(|f| sanitize_path(f.clone())).collect();

    let target;

    if remir {
        target = BuildTargetMode::Remir;
    } else {
        target = BuildTargetMode::VendorIR;
    }

    setup_build_state(PathBuf::from("."), target, input, "".to_string(), false);
    build();
}

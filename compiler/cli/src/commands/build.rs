use std::path::PathBuf;

use calsc_builder::{build, setup_build_state};
use calsc_state::{GLOBAL_STATE, build::BuildTargetMode};

pub fn sanitize_path(path: PathBuf) -> PathBuf {
    let s = path.to_string_lossy();

    let trimmed = s.trim_start_matches(' ');

    PathBuf::from(trimmed)
}

pub fn build_command(
    input: PathBuf,
    out: PathBuf,
    linker: String,
    use_pie: bool,
    package_name: String,
) {
    let out = sanitize_path(out);
    let input = sanitize_path(input);

    setup_build_state(out, BuildTargetMode::Executable, input, linker, use_pie);
    GLOBAL_STATE.with_borrow_mut(|f| {
        f.package_name = package_name.into();
        f.is_package_enabled = true;
    });

    build();
}

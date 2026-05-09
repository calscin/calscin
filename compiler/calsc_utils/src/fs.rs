//! File utilities

use std::fs;

use crate::pos::FilePosition;

/// Get the lines between the two given file positions. Only works if both positions are from the same file.
///
/// # Warn
/// Will panic if the two `FilePosition` have different file paths.
///
pub fn get_line_between_positions(
    start: FilePosition,
    end: FilePosition,
) -> Result<Vec<String>, std::io::Error> {
    assert!(start.file_path == end.file_path);

    let lines: Vec<String> = fs::read_to_string(start.file_path)?
        .lines()
        .map(|line| line.to_string())
        .collect();

    Ok(lines[start.line..end.line + 1].to_vec())
}

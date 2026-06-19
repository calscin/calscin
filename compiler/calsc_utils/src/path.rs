use std::{env, io, path::PathBuf};

pub fn to_absolute_path(path: PathBuf) -> io::Result<PathBuf> {
    let absolute_path = if path.is_absolute() {
        path
    } else {
        env::current_dir()?.join(path)
    };

    Ok(absolute_path)
}

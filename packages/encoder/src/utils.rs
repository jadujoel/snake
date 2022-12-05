use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use path_clean::PathClean;

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }
    .clean();

    Ok(absolute_path)
}

pub fn read_dir(path: &str, extension: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();

    let mut files = vec![];
    for path in paths {
        let path = path.unwrap().path();
        let absolute_path = absolute_path(path).unwrap().display().to_string();

        // if file ends with extension .wav
        if absolute_path.ends_with(extension) {
            files.push(absolute_path);
        }
    }
    files
}

pub fn get_wavs(dir: &str) -> Vec<String> {
    let dir = absolute_path(dir).unwrap().display().to_string();

    read_dir(&dir, "wav")
}

pub fn get_absolute(dir: &str) -> String {
    absolute_path(dir).unwrap().display().to_string()
}

pub fn create_dir(dir: &str) {
    if let Err(_err) = fs::create_dir(dir) {
        // we don't care if the directory already exists, thats fine
    }
}

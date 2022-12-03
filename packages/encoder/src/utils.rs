use std::env;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};

use path_clean::PathClean;

pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

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
    return files;
}

// https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

use std::fs::{File, OpenOptions};
use std::path::Path;
use std::{fs, io};

pub fn open_file(filename: &str) -> io::Result<File> {
    let path = Path::new(&filename);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)?;

    Ok(file)
}

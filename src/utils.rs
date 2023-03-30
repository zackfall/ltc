use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

pub fn open_file(path: &Path) -> io::Result<File> {
    if path.exists() {
        OpenOptions::new().append(true).read(true).open(path)
    } else {
        OpenOptions::new()
            .write(true)
            .read(true)
            .create_new(true)
            .open(path)
    }
}

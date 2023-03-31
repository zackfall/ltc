use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

/// Function to open a file of created in the case that the file doesn't exists.
///
/// Returns:
/// It should return a File in any case, or an Io Error.
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

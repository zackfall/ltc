use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

/// Function to open a file of created in the case that the file doesn't exists.
///
/// # Errors
/// Returns a io error in case it can't open or create the file
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

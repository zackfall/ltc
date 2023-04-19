use std::{
    fs::{File, OpenOptions},
    io,
    path::Path,
};

/// Function to open a file or create it in the case that the file does not exists.
///
/// # Errors
/// Returns an io Error in case it can't open or create the file
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

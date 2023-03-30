use std::{
    fs::{File, OpenOptions},
    io::{self, Read},
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

pub fn get_file_content(mut file: File) -> io::Result<String> {
    let meta = file.metadata()?;
    if meta.len() != 0 {
        let mut output = String::new();
        file.read_to_string(&mut output)?;
        Ok(output)
    } else {
        Ok(String::from("The file is empty"))
    }
}

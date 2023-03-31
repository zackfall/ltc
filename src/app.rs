use crate::utils::open_file;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

/// The app struct that will contain configurations and important data for the program
#[derive(Debug)]
pub struct App {
    pub path: &'static Path,
    pub file: File,
}

impl App {
    /// Function to get the content of the file that is stored in the App.
    ///
    /// Returns:
    /// It will should always return a String, but in case of error, will be an Io
    /// Error
    pub fn get_file_content(&mut self) -> io::Result<String> {
        let meta = self.file.metadata()?;
        if meta.len() != 0 {
            let mut output = String::new();
            self.file.read_to_string(&mut output)?;
            Ok(output)
        } else {
            Ok(String::from("The file is empty"))
        }
    }
}

/// Application builder to ease the process of creation and management of the app
pub struct AppBuilder {
    path: &'static Path,
    file: File,
}

impl Default for AppBuilder {
    fn default() -> Self {
        let path = Path::new("./todos.json");
        let file = open_file(path).expect("Should open the file byt the given path");
        Self { path, file }
    }
}

impl AppBuilder {
    /// Build the app with the path and file stored in the builder
    pub fn build(self) -> App {
        App {
            path: self.path,
            file: self.file,
        }
    }

    /// Modify the path stored in the AppBuilder
    pub fn path(&mut self, path: &'static Path) -> &mut AppBuilder {
        self.path = path;
        self
    }

    /// Modify the file stored in the AppBuilder
    pub fn file(&mut self, file: File) -> &mut AppBuilder {
        self.file = file;
        self
    }
}

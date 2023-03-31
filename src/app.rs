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
    /// # Errors
    /// It could return a Io Error in case the file doesn't exists or,
    /// if the data in the file is no valid UTF
    pub fn get_file_content(&mut self) -> io::Result<String> {
        let meta = self.file.metadata()?;
        if meta.len() == 0 {
            Ok(String::from("The file is empty"))
        } else {
            let mut output = String::new();
            self.file.read_to_string(&mut output)?;
            Ok(output)
        }
    }
}

/// Application builder to ease the process of creation and management of the app
pub struct Builder {
    file_path: &'static Path,
    todos_file: File,
}

impl Default for Builder {
    fn default() -> Self {
        let path = Path::new("./todos.json");
        let file = open_file(path).expect("Should open the file byt the given path");
        Self {
            file_path: path,
            todos_file: file,
        }
    }
}

impl Builder {
    /// Build the app with the path and file stored in the builder
    #[must_use]
    pub fn build(self) -> App {
        App {
            path: self.file_path,
            file: self.todos_file,
        }
    }

    /// Modify the path stored in the `AppBuilder`
    pub fn path(&mut self, path: &'static Path) -> &mut Builder {
        self.file_path = path;
        self
    }

    /// Modify the file stored in the `AppBuilder`
    pub fn file(&mut self, file: File) -> &mut Builder {
        self.todos_file = file;
        self
    }
}

use crate::utils::open_file;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

/// The app struct that will contain configurations and important data for the program
///
/// # Example
/// ```rust
/// use std::path::Path;
/// use std::fs::{File, remove_file};
/// use ltc::app::App;
///
/// let test_path = Path::new("./test-todo.json");
/// let test_file = File::create(test_path).expect("Should return a new file");
/// let mut app = App::default();
/// app.set_path(test_path).set_file(test_file);
/// let file_content = app.get_file_content().expect("Should return an empty string");
/// assert_eq!(file_content, "The file is empty".to_owned());
/// remove_file(test_path);
/// ```
#[derive(Debug)]
pub struct App {
    path: &'static Path,
    file: File,
}

impl Default for App {
    fn default() -> Self {
        let path = Path::new("./todos.json");
        let file = open_file(path).expect("Should open the file byt the given path");
        Self {
            path,
            file,
        }
    }
}

impl App {
    /// Function to get the content of the file that is stored in the App.
    ///
    /// # Errors
    /// It could return an Io Error in case the file doesn't exists or,
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

    pub fn set_path(&mut self, path: &'static Path) -> &mut App {
        self.path = path;
        self
    }

    pub fn set_file(&mut self, file: File) -> &mut App {
        self.file = file;
        self
    }
}

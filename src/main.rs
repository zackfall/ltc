use ltc::utils::*;
use std::{io, path::Path};

fn main() -> io::Result<()> {
    let path = Path::new("./todos.json");
    let file = open_file(path)?;
    let file_content = get_file_content(file)?;
    println!("{file_content}");
    Ok(())
}

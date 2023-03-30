use ltc::utils::*;
use std::{io, path::Path};

fn main() -> io::Result<()> {
    println!("Welcome, this is a placeholder text");
    let file = open_file(Path::new("./todos.json"))?;
    println!("Now I will print the file {:?}", file);
    Ok(())
}

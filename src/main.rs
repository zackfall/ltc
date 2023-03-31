use std::io;

use ltc::app::Builder;

fn main() -> io::Result<()> {
    let mut app = Builder::default().build();
    let file_content = app.get_file_content()?;
    println!("{file_content}");
    Ok(())
}

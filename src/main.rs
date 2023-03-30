use std::io;

use ltc::app::AppBuilder;

fn main() -> io::Result<()> {
    let mut app = AppBuilder::default().build();
    let file_content = app.get_file_content()?;
    println!("{file_content}");
    Ok(())
}

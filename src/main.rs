use std::io;

use ltc::app::App;

fn main() -> io::Result<()> {
    let mut app = App::default();
    let file_content = app.get_file_content()?;
    println!("{file_content}");
    Ok(())
}

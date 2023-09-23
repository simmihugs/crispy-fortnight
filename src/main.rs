mod currentline;
mod debug;
mod keys;
mod my_parser;

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use keys::read_char;
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = read_char() {
        println!("Error: {:?}\r", e);
    }

    debug::debug_clear()?;

    io::stdout().execute(cursor::SetCursorStyle::DefaultUserShape)?;
    execute!(stdout, DisableMouseCapture)?;
    println!("Bye {}!", "😁");

    disable_raw_mode()
}

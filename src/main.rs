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
    let x = match cursor::position() {
        Ok((x, _)) => x,
        _ => 0,
    };
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = read_char() {
        println!("Error: {:?}\r", e);
    }

    let (xp, yp) = match cursor::position() {
        Ok((x, y)) => (x, y),
        _ => (0, 0),
    };
    println!("\rBye {}!", "😁");
    debug::debug_clear()?;

    io::stdout().execute(cursor::SetCursorStyle::DefaultUserShape)?;
    execute!(stdout, DisableMouseCapture)?;

    if x == 0 {
        io::stdout().execute(cursor::MoveTo(xp, yp))?;
    } else {
        io::stdout().execute(cursor::MoveTo(x, yp))?;
    }

    disable_raw_mode()
}

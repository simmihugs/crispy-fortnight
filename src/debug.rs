use std::io::{self, Write};

use crossterm::{
    cursor,
    event::Event,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use super::currentline::CurrentLine;

pub fn debug_message(message: &str) -> io::Result<()> {
    match cursor::position() {
        Ok((x, y)) => {
            io::stdout().execute(cursor::MoveTo(0, 16))?;
            println!("\rmessage: {}", message);
            io::stdout().flush()?;
            io::stdout().execute(cursor::MoveTo(x, y))?;
        }
        _ => (),
    }
    Ok(())
}

pub fn debug_line(line: &mut CurrentLine) -> io::Result<()> {
    match cursor::position() {
        Ok((x, y)) => {
            io::stdout().execute(cursor::MoveTo(0, 15))?;
            println!("\rcurrentline {:?}", line);
            io::stdout().flush()?;
            io::stdout().execute(cursor::MoveTo(x, y))?;
        }
        _ => (),
    }
    Ok(())
}

#[allow(dead_code)]
pub fn debug_event(event: &Event) -> io::Result<()> {
    match cursor::position() {
        Ok((x, y)) => {
            io::stdout().execute(cursor::MoveTo(0, 15))?;
            println!("\revent {:?}", event);
            io::stdout().flush()?;
            io::stdout().execute(cursor::MoveTo(x, y))?;
        }
        _ => (),
    }
    Ok(())
}

pub fn debug_clear() -> io::Result<()> {
    match cursor::position() {
        Ok((x, y)) => {
            io::stdout().execute(cursor::MoveTo(0, 15))?;
            io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
            io::stdout().execute(cursor::MoveTo(0, 16))?;
            io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
            io::stdout().execute(cursor::MoveTo(x, y))?;
        }
        _ => (),
    }
    Ok(())
}

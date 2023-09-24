use std::io::{self, Write};

use crossterm::{
    cursor,
    event::Event,
    style,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use super::currentline::CurrentLine;

pub fn debug_message(message: &str) -> io::Result<()> {
    match cursor::position() {
        Ok((x, y)) => match terminal::size() {
            Ok((width, height)) => {
                let local_message = format!("DEBUG: message: {}", message);
                io::stdout().execute(cursor::MoveTo(0, height - 3))?;
                io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                io::stdout().execute(style::SetBackgroundColor(style::Color::Cyan))?;
                io::stdout().execute(style::SetForegroundColor(style::Color::Black))?;
                print!("\r{:width$}", local_message, width = width as usize);
                io::stdout().flush()?;
                io::stdout().execute(cursor::MoveTo(x, y))?;
                io::stdout().execute(style::ResetColor)?;
            }
            _ => (),
        },
        _ => (),
    }
    Ok(())
}

#[allow(dead_code)]
pub fn debug_event(event: &Event) -> io::Result<()> {
    match cursor::position() {
        Ok((x, y)) => match terminal::size() {
            Ok((width, height)) => {
                let message = format!("DEBUG: event: {:?}", event);
                io::stdout().execute(cursor::MoveTo(0, height - 2))?;
                io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                io::stdout().execute(style::SetBackgroundColor(style::Color::Green))?;
                io::stdout().execute(style::SetForegroundColor(style::Color::Black))?;
                print!("\r{:width$}", message, width = width as usize);
                io::stdout().flush()?;
                io::stdout().execute(cursor::MoveTo(x, y))?;
                io::stdout().execute(style::ResetColor)?;
            }
            _ => (),
        },
        _ => (),
    }
    Ok(())
}

pub fn debug_line(line: &mut CurrentLine) -> io::Result<()> {
    match cursor::position() {
        Ok((x, y)) => match terminal::size() {
            Ok((width, height)) => {
                let message = format!("DEBUG: currentline {:?}", line);
                io::stdout().execute(cursor::MoveTo(0, height - 1))?;
                io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                io::stdout().execute(style::SetBackgroundColor(style::Color::Magenta))?;
                io::stdout().execute(style::SetForegroundColor(style::Color::Black))?;
                print!("\r{:width$}", message, width = width as usize);
                io::stdout().flush()?;
                io::stdout().execute(cursor::MoveTo(x, y))?;
                io::stdout().execute(style::ResetColor)?;
            }
            _ => (),
        },
        _ => (),
    }
    Ok(())
}

pub fn debug_clear() -> io::Result<()> {
    match terminal::size() {
        Ok((_, height)) => {
            for i in 1..=3 {
                io::stdout().execute(cursor::MoveTo(0, height - i))?;
                io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
            }
            io::stdout().execute(style::ResetColor)?;
        }
        _ => (),
    }
    Ok(())
}

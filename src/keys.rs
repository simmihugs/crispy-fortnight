use super::currentline::CurrentLine;
use super::debug::{debug_event, debug_line, debug_message};
use super::my_parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

fn regular_character(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
        ..
    }) = event
    {
        line.add_char(c.clone())?;
    }

    Ok(())
}
fn control_l(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('l'),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event
    {
        line.clear();
        io::stdout().execute(cursor::MoveTo(0, 0))?;
        io::stdout().execute(terminal::Clear(ClearType::All))?;
        print!("\r> ");
        io::stdout().flush()?;
        line.set_position(2, 0);
    }

    Ok(())
}
fn control_k(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('k'),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event
    {
        match cursor::position() {
            Ok((_, y)) => {
                io::stdout().execute(cursor::MoveTo(2, y))?;
                io::stdout().execute(terminal::Clear(ClearType::UntilNewLine))?;
                line.update_position(0, y);
                line.clear();
                line.display()?;
            }
            _ => (),
        }
    }

    Ok(())
}
fn control_a(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('a'),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event
    {
        debug_line(line)?;
        match cursor::position() {
            Ok((_, y)) => {
                io::stdout().execute(cursor::MoveTo(0, y))?;
                io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                print!("\r> ");
                io::stdout().flush()?;
                line.update_position(2, y);
                line.display()?;
                debug_line(line)?;
            }
            _ => (),
        }
    }

    Ok(())
}
fn control_b(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('b'),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event
    {
        debug_line(line)?;
        debug_event(event)?;
        match cursor::position() {
            Ok((x, y)) => {
                if x > 2 {
                    io::stdout().execute(cursor::MoveTo(x - 1, y))?;
                    io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                    print!("\r> ");
                    io::stdout().flush()?;
                    line.left();
                    line.display()?;
                    debug_line(line)?;
                }
            }
            _ => (),
        }
    }
    Ok(())
}
fn control_f(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('f'),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event
    {
        debug_line(line)?;
        debug_event(event)?;
        match cursor::position() {
            Ok((x, y)) => {
                if x > 2 {
                    io::stdout().execute(cursor::MoveTo(x + 1, y))?;
                    io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                    print!("\r> ");
                    io::stdout().flush()?;
                    line.right();
                    line.display()?;
                    debug_line(line)?;
                }
            }
            _ => (),
        }
    }
    Ok(())
}
fn parse_line(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Enter,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        let mut parse_result = String::from("");
        match my_parser::parse(line.collect()) {
            Err(result) => {
                if result == "quit" {
                    debug_message("Quit")?;
                    return Err(io::Error::from(io::ErrorKind::Interrupted));
                } else {
                    parse_result = result;
                }
            }
            Ok(s) => {
                parse_result = format!("{}\n", s);
            }
        }
        if parse_result.trim() == "" {
            parse_result = String::from("Could not parse\n");
        }

        print!("\n\r{}\r> ", parse_result);
        line.clear();
        line.position_down();
        if parse_result.contains("\n") {
            line.position_down();
        }
        io::stdout().flush()?;
    }
    Ok(())
}
fn backspace(event: &Event, line: &mut CurrentLine) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Backspace,
        kind: KeyEventKind::Press,
        ..
    }) = event
    {
        line.delete_left()?;
        line.display()?;
    }

    Ok(())
}
fn control_c(event: &Event) -> io::Result<()> {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char('c'),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::CONTROL,
        ..
    }) = event
    {
        return Err(io::Error::from(io::ErrorKind::Interrupted));
    }

    Ok(())
}
fn prompt() -> io::Result<()> {
    print!("\r> ");
    io::stdout().flush()?;

    Ok(())
}
pub fn read_char() -> io::Result<()> {
    io::stdout().execute(cursor::SetCursorStyle::BlinkingBlock)?;
    println!("Welcome to the crispy repl {}!", "😁");
    prompt()?;

    let (x, y) = match cursor::position() {
        Ok((_x, _y)) => (_x, _y),
        _ => (2, 0),
    };
    let mut line = CurrentLine::new(x, y);

    loop {
        match event::read() {
            Err(..) => (),
            Ok(event) => {
                regular_character(&event, &mut line)?;
                backspace(&event, &mut line)?;
                control_k(&event, &mut line)?;
                control_l(&event, &mut line)?;
                control_a(&event, &mut line)?;
                control_b(&event, &mut line)?;
                control_f(&event, &mut line)?;
                match parse_line(&event, &mut line) {
                    Err(..) => break,
                    _ => (),
                }
                match control_c(&event) {
                    Err(..) => break,
                    _ => (),
                }
            }
        }
    }

    Ok(())
}

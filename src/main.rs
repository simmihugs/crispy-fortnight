use crossterm::{
    cursor,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    ExecutableCommand,
};
use std::io::{self, stdout, Write};

mod my_parser {
    use std::io::*;

    pub fn print_help() -> Result<()> {
        print!("\nHelp!");
        stdout().flush()?;

        Ok(())
    }

    pub fn parse(string: String) -> Result<String> {
        if string == ":h" {
            match print_help() {
                _ => (),
            }

            Ok("help".to_string())
        } else if string.contains(":quit") || string.contains(":q") {
            Ok("quit".to_string())
        } else {
            let mut result = String::new();
            let params: Vec<String> = string.split(' ').map(|x| x.to_string()).collect();
            for (i, p) in params.iter().enumerate() {
                if p.contains(":") && i < params.len() - 1 {
                    result += &format!(
                        "{}{{key: {:?}, value: {:?}}}",
                        if i != 0 { ",\t" } else { "" },
                        p.replace(":", ""),
                        params[i + 1]
                    );
                }
            }

            Ok(result)
        }
    }
}

pub fn read_char() -> io::Result<()> {
    let mut line = String::new();
    print!("\r> ");
    io::stdout().flush()?;
    loop {
        match event::read() {
            Err(..) => (),
            Ok(event) => {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: KeyEventKind::Press,
                    ..
                }) = event
                {
                    line.push(c);
                    print!("{}", c);
                    io::stdout().flush()?;
                } else if let Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: KeyEventKind::Press,
                    ..
                }) = event
                {
                    let mut parse_result = String::new();
                    match my_parser::parse(line) {
                        Ok(s) => {
                            if s.contains("quit") {
                                break;
                            } else if s != "" {
                                parse_result = format!("{}\n", s);
                            }
                        }
                        _ => (),
                    }

                    print!("\n\r{}\r> ", parse_result);
                    line = String::new();
                    io::stdout().flush()?;
                } else if let Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) = event
                {
                    break;
                } else if let Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) = event
                {
                    match cursor::position() {
                        Ok((x, _)) => {
                            if x > 2 {
                                io::stdout().execute(cursor::MoveLeft(1))?;
                                io::stdout().execute(terminal::Clear(ClearType::UntilNewLine))?;
                                if line.len() != 0 {
                                    line.pop();
                                }
                            }
                        }
                        _ => (),
                    }
                } else if let Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    kind: KeyEventKind::Release,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) = event
                {
                    break;
                } else if let Event::Key(KeyEvent {
                    code: KeyCode::Char('l'),
                    kind: KeyEventKind::Release,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) = event
                {
                    line = String::new();
                    io::stdout().execute(cursor::MoveTo(0, 0))?;
                    io::stdout().execute(terminal::Clear(ClearType::All))?;
                    print!("\n\r> ");
                    io::stdout().flush()?;
                } else if let Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    kind: KeyEventKind::Release,
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) = event
                {
                    line = String::new();
                    match cursor::position() {
                        Ok((_, y)) => {
                            io::stdout().execute(cursor::MoveTo(0, y))?;
                            io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                            print!("\r> ");
                            io::stdout().flush()?;
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = read_char() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}

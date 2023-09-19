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
use std::io::{self, Write};

mod my_parser {
    pub fn parse(string: String) -> String {
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

        result
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
                    let mut parse_result = my_parser::parse(line);
                    if parse_result != "" {
                        parse_result += "\n";
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

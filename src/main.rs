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

struct Position {
    x: u16,
    y: u16,
}
impl Position {
    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn down(&mut self) {
        self.y += 1;
    }
}
struct CurrentLine {
    position: Position,
    leftbuffer: String,
    rightbuffer: String,
}

impl CurrentLine {
    #[allow(dead_code)]
    pub fn pop_left(&mut self) {
        self.leftbuffer.pop();
    }

    pub fn collect(&self) -> String {
        format!("{}{}", self.leftbuffer, self.rightbuffer)
    }

    pub fn clear(&mut self) {
        self.set_position(2, self.position.y);
        self.leftbuffer = String::new();
        self.rightbuffer = String::new();
    }

    pub fn new(x: u16, y: u16) -> Self {
        CurrentLine {
            position: Position { x, y },
            leftbuffer: String::new(),
            rightbuffer: String::new(),
        }
    }

    pub fn position_down(&mut self) {
        self.position.down();
    }

    pub fn delete_left(&mut self) -> io::Result<()> {
        self.leftbuffer.pop();
        self.position.left();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_right(&mut self) -> io::Result<()> {
        self.rightbuffer = self.rightbuffer.drain(1..).collect::<String>();

        Ok(())
    }

    pub fn add_char(&mut self, c: char) -> io::Result<()> {
        self.position.x += 1;
        assert!(self.position.x >= 2);
        self.leftbuffer.push(c);
        self.display()
    }

    pub fn display(&self) -> io::Result<()> {
        io::stdout().execute(cursor::MoveTo(2, self.position.y()))?;
        io::stdout().execute(terminal::Clear(ClearType::UntilNewLine))?;
        print!("\r> {}{}", self.leftbuffer, self.rightbuffer);
        io::stdout().flush()?;
        io::stdout().execute(cursor::MoveTo(self.position.x(), self.position.y()))?;

        Ok(())
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        self.position = Position { x, y };
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        self.set_position(x, y);
        let collection = self.collect();
        let (leftbuffer, rightbuffer) = collection.split_at(x.into());
        self.leftbuffer = leftbuffer.to_string();
        self.rightbuffer = rightbuffer.to_string();
    }
}

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
                /*                 print!("\r> ");
                               io::stdout().flush()?;
                */
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
        match cursor::position() {
            Ok((_, y)) => {
                io::stdout().execute(cursor::MoveTo(0, y))?;
                io::stdout().execute(terminal::Clear(ClearType::CurrentLine))?;
                print!("\r> ");
                io::stdout().flush()?;
                line.update_position(2, y);
                line.display()?;
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
        let mut parse_result = String::new();
        match my_parser::parse(line.collect()) {
            Ok(s) => {
                if s.contains("quit") {
                    return Err(io::Error::from(io::ErrorKind::Interrupted));
                } else if s != "" {
                    parse_result = format!("{}\n", s);
                } else {
                    parse_result = format!("{}\n", "could not parse");
                }
            }
            _ => (),
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

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = read_char() {
        println!("Error: {:?}\r", e);
    }

    io::stdout().execute(cursor::SetCursorStyle::DefaultUserShape)?;
    execute!(stdout, DisableMouseCapture)?;
    println!("Bye {}!", "😁");
    
    disable_raw_mode()
}

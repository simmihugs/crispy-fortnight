use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

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

pub struct CurrentLine {
    position: Position,
    leftbuffer: String,
    rightbuffer: String,
}
impl CurrentLine {
    #[allow(dead_code)]
    pub fn pop_left(&mut self) {
        self.leftbuffer.pop();
    }

    pub fn left(&mut self) {
        self.position.left();
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

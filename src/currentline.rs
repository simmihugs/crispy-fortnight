//use super::debug;
use crossterm::{
    cursor,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, Write};

#[derive(Debug)]
pub struct Position {
    x: u16,
    y: u16,
}
impl Position {
    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn down(&mut self) {
        self.y += 1;
    }
}

#[derive(Debug)]
pub struct CurrentLine {
    pub position: Position,
    leftbuffer: String,
    rightbuffer: String,
}
impl CurrentLine {
    #[allow(dead_code)]
    pub fn pop_left(&mut self) {
        self.leftbuffer.pop();
    }

    /*     pub fn right(&mut self) {
        if self.rightbuffer != "" {
            match debug::debug_message("go right") {
                _ => (),
            }

            self.position.move_right();
        } else {
            match debug::debug_message("cannot go right") {
                _ => (),
            }
        }
    } */

    /*     pub fn left(&mut self) {
        self.position.move_left();
    } */

    pub fn collect(&self) -> String {
        format!("{}{}", self.leftbuffer, self.rightbuffer)
    }

    pub fn clear(&mut self) {
        //self.set_position(2, self.position.y);
        //self.position.set_x(0);
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
        self.position.move_left();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_right(&mut self) -> io::Result<()> {
        self.rightbuffer = self.rightbuffer.drain(1..).collect::<String>();

        Ok(())
    }

    pub fn add_char(&mut self, c: char) -> io::Result<()> {
        self.position.x += 1;
        self.leftbuffer.push(c);
        self.display()
    }

    pub fn display(&self) -> io::Result<()> {
        io::stdout().execute(cursor::MoveTo(2, self.position.y()))?;
        io::stdout().execute(terminal::Clear(ClearType::UntilNewLine))?;
        print!("\r> {}{}", self.leftbuffer, self.rightbuffer);
        io::stdout().flush()?;
        io::stdout().execute(cursor::MoveTo(self.position.x() + 2, self.position.y()))?;

        Ok(())
    }

    pub fn clear_rightbuffer(&mut self) {
        self.rightbuffer = String::new();
    }

    pub fn set_position_start_x(&mut self) {
        self.position.set_x(0);
        self.rightbuffer = self.collect();
        self.leftbuffer = String::new();
    }

    pub fn set_position_start_y(&mut self) {
        self.position.set_y(0);
    }

    pub fn set_position_end(&mut self) {
        self.leftbuffer = self.collect();
        self.position.set_x(self.leftbuffer.len() as u16);
        self.rightbuffer = String::new();
    }

    pub fn move_left(&mut self) -> bool {
        if self.position.x == 0 {
            false
        } else {
            self.position.move_left();
            let collection = self.collect();
            let (leftbuffer, rightbuffer) = collection.split_at(self.position.x.into());
            self.leftbuffer = leftbuffer.to_string();
            self.rightbuffer = rightbuffer.to_string();
            true
        }
    }

    pub fn length(&self) -> usize {
        self.leftbuffer.len() + self.rightbuffer.len()
    }

    pub fn move_right(&mut self) -> bool {
        if self.position.x == self.length() as u16 {
            false
        } else {
            self.position.move_right();
            let collection = self.collect();
            let (leftbuffer, rightbuffer) = collection.split_at(self.position.x.into());
            self.leftbuffer = leftbuffer.to_string();
            self.rightbuffer = rightbuffer.to_string();
            true
        }
    }
}

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

    pub fn collect(&self) -> String {
        format!("{}{}", self.leftbuffer, self.rightbuffer)
    }

    pub fn clear(&mut self) {
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

    pub fn delete_left(&mut self) {
        self.leftbuffer.pop();
        self.position.move_left();
    }

    pub fn delete_right(&mut self) {
        if self.rightbuffer.len() > 0 {
            self.rightbuffer = self.rightbuffer.drain(1..).collect::<String>();
            super::debug::debug_message("Delete one character from right buffer").unwrap();
        } else {
            super::debug::debug_message("Right buffer empty").unwrap();
        }
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

    pub fn set_position_x(&mut self, x: u16) {
        let collection = self.collect();
        let (left, right) = collection.split_at(x.into());
        self.leftbuffer = left.to_string();
        self.rightbuffer = right.to_string();
        self.position.set_x(x);
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

    pub fn delete_word_right(&mut self) {
        if self.rightbuffer.len() > 0 {
            match self.rightbuffer.find(' ') {
                Some(index) => {
                    let mut real_index = index;
                    while real_index < self.rightbuffer.len() {
                        if self.rightbuffer.chars().nth(real_index) != Some(' ') {
                            break;
                        }
                        real_index += 1;
                    }
                    super::debug::debug_message(
                        format!("found space in right buffer: {}", index).as_str(),
                    )
                    .unwrap();
                    self.rightbuffer = self.rightbuffer.drain(real_index..).collect::<String>();
                }
                _ => super::debug::debug_message("found space in right buffer").unwrap(),
            }
        }
    }

    pub fn left_word(&self) -> Option<u16> {
        if self.position.x() == 0 {
            None
        } else {
            let mut c = Vec::new();
            self.collect().split(' ').fold(0, |mut acc, value| {
                c.push(acc);
                acc += value.to_string().len();
                acc
            });

            let collection = self.collect();
            let char_indeces = collection
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == ' ')
                .collect::<Vec<_>>();

            let mut res = vec![0];
            char_indeces.iter().for_each(|(i, _)| {
                let chars = collection.chars().collect::<Vec<_>>();
                if chars.len() > i + 1 {
                    let next = chars[i + 1];
                    if next != ' ' {
                        res.push(i + 1);
                    }
                }
            });
            match res
                .iter()
                .filter(|&&i| i < self.position.x() as usize)
                .max()
            {
                Some(i) => Some(*i as u16),
                _ => None,
            }
        }
    }
    pub fn right_word(&self) -> Option<u16> {
        let mut c = Vec::new();
        self.collect().split(' ').fold(0, |mut acc, value| {
            c.push(acc);
            acc += value.to_string().len();
            acc
        });

        let collection = self.collect();
        let char_indeces = collection
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == ' ')
            .collect::<Vec<_>>();

        let mut res = vec![0];
        char_indeces.iter().for_each(|(i, _)| {
            let chars = collection.chars().collect::<Vec<_>>();
            if chars.len() > i + 1 {
                let next = chars[i + 1];
                if next != ' ' {
                    res.push(i + 1);
                }
            }
        });
        match res
            .iter()
            .filter(|&&i| i > self.position.x() as usize)
            .min()
        {
            Some(i) => Some(*i as u16),
            _ => Some(self.length() as u16),
        }
    }
}

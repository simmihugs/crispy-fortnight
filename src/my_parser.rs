use super::debug::debug_message;
use std::io::{self, Write};

pub fn print_help() -> io::Result<()> {
    print!("\nHelp!");
    debug_message("Print Help!")?;
    io::stdout().flush()?;

    Ok(())
}

pub enum Command {
    Help,
    Quit,
    Load(String),
    Invalid,
    Command(Box<Command>),
}

#[derive(Debug)]
struct CommandMap {
    key: String,
    arg: Vec<String>,
}
impl CommandMap {
    fn new(name: &str) -> Self {
        Self {
            key: String::from(name),
            arg: Vec::new(),
        }
    }

    fn add(&mut self, value: &str) {
        let tmp = String::from(value.trim());
        if tmp != "" {
            self.arg.push(tmp);
        }
    }

    fn to_string(&self) -> String {
        if self.arg.len() == 0 {
            format!("{{key: {}}}", self.key)
        } else {
            let args: String = self.arg.iter().fold(String::from(""), |mut acc, value| {
                acc += &value;
                acc += " ";
                acc
            });
            format!(
                "{{key: {}, {}: {}}}",
                self.key,
                if self.arg.len() == 1 { "arg" } else { "args" },
                args
            )
        }
    }
}
pub fn parse(string: String) -> Command {
    if string == ":h" {
        Command::Help
    } else if string.contains(":quit") || string.contains(":q") {
        Command::Quit
    } else {
        let mut keys: Vec<CommandMap> = Vec::new();
        string
            .split(' ')
            .map(|x| x.to_string())
            .map(|x| {
                if x.contains(":") {
                    x.split(":").map(|x| x.to_string()).collect::<Vec<String>>()
                } else {
                    vec![x]
                }
            })
            .map(|vec| {
                if vec.len() == 1 {
                    vec![("arg", vec[0].to_string())]
                } else if vec[0] == "" {
                    vec![("key", vec[1].to_string())]
                } else {
                    vec![("arg", vec[0].to_string()), ("key", vec[1].to_string())]
                }
            })
            .flatten()
            .for_each(|(s, v)| {
                if s == "key" {
                    keys.push(CommandMap::new(&v));
                } else {
                    if keys.len() > 0 {
                        let last_index = keys.len() - 1;
                        keys[last_index].add(&v);
                    }
                }
            });

        let result = keys
            .iter()
            .map(|x| x.to_string())
            .fold(String::from(""), |mut acc, value| {
                acc += &value;
                acc
            });

        if result == "" {
            Command::Invalid
        } else {
            Command::Load(result)
        }
    }
}

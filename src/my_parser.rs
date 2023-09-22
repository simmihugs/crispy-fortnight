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

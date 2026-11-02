use std::fs::File;
use std::io::{self, Write};

use crate::error::ShellErr;

pub fn get_user_input() -> Result<String, ShellErr> {
    let mut line = String::new();
    let stdin = io::stdin();

    // Read a string (command) from user
    let bytes_read = stdin.read_line(&mut line)?;

    //Check if CTRL+D or EOF
    if bytes_read == 0 { 
        return Err(ShellErr::EmptyInput);
    }

    Ok(line)
}

pub fn edit_buf(filepath: &str) -> Result<(), ShellErr> {
    let mut file = File::create(filepath)?;
    let mut write_buf = String::new();

    let terminator = String::from("q\n");

    let mut line = get_user_input()?; 
    while line != terminator {
        write_buf.push_str(&line);
        line.clear();
        line = get_user_input()?;
    }

    write!(file, "{}", write_buf)?;

    Ok(())
}
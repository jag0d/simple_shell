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
    let stdin = io::stdin();
    let mut line = String::new();

    while let read_bytes = stdin.read_line(&mut line)? != 0 {
        
    }

    Ok(())
}
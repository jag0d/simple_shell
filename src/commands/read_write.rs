use std::fs::File;
use std::io::{self, Write};

use crate::error::ShellErr;

// Helper for reading a line of input from user
pub fn get_user_input() -> Result<Option<String>, ShellErr> {
    let mut line = String::new();
    let stdin = io::stdin();

    let bytes_read = stdin.read_line(&mut line)?;

    // Check for EOF. This is not treated as an error.
    // Exit quietly
    if bytes_read == 0 { 
        return Ok(None);
    }

    // Return exactly one line of raw input
    Ok(Some(line))
}

// Write text line-by-line to a file
pub fn write_buf(filepath: &str) -> Result<(), ShellErr> {
    // If file exists -> overwrite
    // If file doesn't exist -> create
    // If path invalid or failed to create -> io::Error
    let mut file = File::create(filepath)?;
    let mut write_buf = String::new();

    // Will append to write_buf until "q" or EOF
    loop {
        let Some(line) = get_user_input()? else {
            break;
        };

        if line.trim() == "q" {
            break;
        }
        write_buf.push_str(&line);
    }

    // Write entire buffer to file
    write!(file, "{}", write_buf)?;

    Ok(())
}
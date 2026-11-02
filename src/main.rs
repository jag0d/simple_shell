mod error;
mod commands;
mod parser;

use std::io::{self, Write};

use crate::{parser::Command};

fn main() -> std::io::Result<()> {

    let mut cmd_buf;
    
    // Main program loop: Display "$ " -> get user input -> parse command -> execute command
    loop {
        // Display shell prompt and take a line of user input
        print!("$ ");
        io::stdout().flush()?;

        // Will exit program upon EOF (CTRL+D)
        // Will continue if only \n is passed
        match commands::write::get_user_input() {
            Ok(Some(n)) => { cmd_buf = n },
            Ok(None) => break,
            Err(err) => {
                eprintln!("shell: {}", err);
                continue;
            },
        }

        //Trim the input and remove a preceeding "$ " if there is one
        let cmd_buf_trim = cmd_buf.trim();
        let cmd = cmd_buf_trim.strip_prefix("$ ").unwrap_or(cmd_buf_trim);

        // Parse user input into one of implemented commands
        // Execute command if matched
        // Handle any error bubbled from functions
        match Command::parse_command(&cmd) {
            Ok(Command::Exit) => break,
            Ok(command) => {
                match parser::execute_command(command) {
                    Ok(()) => continue,
                    Err(err) => eprintln!("shell: {}", err),
                }
            }
            Err(err) => eprintln!("shell: {}", err),
        }
    }

    Ok(())
}

mod error;
mod commands;
mod parser;

use std::io::{self, Write};

use crate::{parser::Command};

fn main() -> std::io::Result<()> {

    let mut cmd_buf;
    
    loop {
        // Display shell prompt and take a line of user input
        print!("$ ");
        io::stdout().flush()?;

        match commands::write::get_user_input() {
            Ok(Some(n)) => { cmd_buf = n },
            Ok(None) => break,
            Err(err) => {
                eprintln!("shell: {}", err);
                continue;
            },
        }

        //Trim the input
        let cmd = cmd_buf.trim();

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

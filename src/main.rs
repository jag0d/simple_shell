mod error;
mod commands;

use std::io::{self, Write};

fn main() -> std::io::Result<()> {

    let mut cmd_buf = String::new();
    let stdin = io::stdin();
    
    loop {
        // Display shell prompt and take a line of user input
        print!("$ ");
        io::stdout().flush()?;

        // Read a string (command) from user
        cmd_buf.clear();
        let bytes_read = match stdin.read_line(&mut cmd_buf) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            },
        };

        //Check if CTRL+D or EOF
        if bytes_read == 0 { 
            break; 
        }

        //Trim the input
        let cmd = cmd_buf.trim();

        //Check if no command entered
        if cmd.is_empty() {
            println!("You did not enter any command.");
            continue;
        }

        //For the time being print the command back to me, if there is any input
        if cmd.eq_ignore_ascii_case("exit") {
            println!("You typed {}. Are you leaving us so soon? Goodbye to you kind sir!", cmd);
            break;
        }
        else if cmd == "pwd" {
            match commands::ops::pwd() {
                Ok(()) => continue,
                Err(err) => eprintln!("shell: {}", err),
            }
        }
        else if cmd == ".." {
            match commands::ops::parent_dir() {
                Ok(()) => continue,
                Err(err) => eprintln!("shell: {}", err),
            }
        }
        else if cmd == "cd" {
            let mut path = String::new();
            stdin.read_line(&mut path).expect("Nah");
            
            match commands::ops::change_dir(&path.trim()) {
                Ok(()) => continue,
                Err(err) => eprintln!("shell: {}", err),
            }

        }
        else if cmd == "ls" {
            match commands::ops::list_dir() {
                Ok(()) => continue,
                Err(err) => eprintln!("shell: {}", err),
            }
        }
        else if cmd == "ls -l" {
            match commands::ops::list_dir_long() {
                Ok(()) => continue,
                Err(err) => eprintln!("shell: {}", err),
            }
        }
        else {
            println!("You seem to have said: {}", cmd);
        }
    }

    Ok(())
}

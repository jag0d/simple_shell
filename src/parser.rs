use crate::error::ShellErr;
use crate::commands::{ops, read_write};

// List of implemented functions and placeholders for exceptions
#[derive(Debug)]
pub enum Command {
    Ls,
    LsLong,
    CdParent,
    Cd(String),
    Pwd,
    WriteFile(String),
    Help,
    Exit,
    NoCommand
}

impl Command {
    // Tokenize command from user and match to supported commands
    pub fn parse_command(cmd_buf: &str) -> Result<Self, ShellErr> {
        let mut cmd= cmd_buf.split_whitespace(); 

        match cmd.next() {
            Some("ls") => {
                if cmd.next() == Some("-l") {
                    Ok(Command::LsLong)
                }
                else {
                    Ok(Command::Ls)
                }
            },

            Some("cd") => {
                let path = cmd.next();

                match path {
                    Some("..") => Ok(Command::CdParent),
                    Some(path) => Ok(Command::Cd(String::from(path))),
                    // If no argument specified for cd stay in current dir
                    None => Ok(Command::Cd(String::from("."))),
                }
            },

            Some("pwd") => Ok(Command::Pwd),

            Some("write") => {
                let path = cmd.next();

                match path {
                    Some(path) => Ok(Command::WriteFile(String::from(path))),
                    // Write needs filepath, but no panics if there's none
                    None => Err(ShellErr::EmptyPath),
                }
            },

            Some("help") => Ok(Command::Help),

            Some("exit") => Ok(Command::Exit),
            // No panics if command not recognized
            Some(bad_cmd) => Err(ShellErr::UnsupportedCommand(String::from(bad_cmd))),
            // Sending only \n will do nothing
            None => Ok(Command::NoCommand),
        }
    }  
}

// Execute parsed command
pub fn execute_command(cmd: Command) -> Result<(), ShellErr> {
    match cmd {
        Command::Ls => ops::list_dir()?,
        Command::LsLong => ops::list_dir_long()?,
        Command::CdParent => ops::parent_dir()?,
        Command::Cd(path) => ops::change_dir(&path)?,
        Command::Pwd => ops::pwd()?,
        Command::WriteFile(path) => read_write::write_buf(&path)?,
        Command::Help => ops::help(),
        // Does nothing (main loop continues)
        Command::NoCommand => (), 
        // Exit handled in main before execute_command() is called
        Command::Exit => (),
    }
    Ok(())
} 


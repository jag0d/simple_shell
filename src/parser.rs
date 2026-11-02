use crate::error::ShellErr;
use crate::commands::{ops, read_write};

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
                    None => Ok(Command::Cd(String::from("."))),
                }
            },

            Some("pwd") => Ok(Command::Pwd),

            Some("write") => {
                let path = cmd.next();

                match path {
                    Some(path) => Ok(Command::WriteFile(String::from(path))),
                    None => Err(ShellErr::EmptyPath),
                }
            },

            Some("help") => Ok(Command::Help),

            Some("exit") => Ok(Command::Exit),

            Some(bad_cmd) => Err(ShellErr::UnsupportedCommand(String::from(bad_cmd))),

            None => Ok(Command::NoCommand),

        }
    }  
}

pub fn execute_command(cmd: Command) -> Result<(), ShellErr> {
    match cmd {
        Command::Ls => ops::list_dir()?,
        Command::LsLong => ops::list_dir_long()?,
        Command::CdParent => ops::parent_dir()?,
        Command::Cd(path) => ops::change_dir(&path)?,
        Command::Pwd => ops::pwd()?,
        Command::WriteFile(path) => read_write::write_buf(&path)?,
        Command::Help => ops::help(),
        Command::NoCommand => (), 
        Command::Exit => (),
    }
    Ok(())
} 


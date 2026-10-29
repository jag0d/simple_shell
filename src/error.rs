use std::fmt::Display;
//use std::error::Error;

//Error handling
pub enum ShellErr {
    EmptyInput,
}

impl Display for ShellErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellErr::EmptyInput  => write!(f, "No user input."),
        }
    }
}
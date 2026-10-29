use std::fmt::Display;
use std::io;
//use std::path::PathBuf;

//Error handling
#[derive(Debug)]
pub enum ShellErr {
    Io(io::Error),
    //DirectoryError(PathBuf),
    //EmptyInput,
}

impl Display for ShellErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellErr::Io(err) => write!(f, "IO error: {}", err),
            //ShellErr::EmptyInput  => write!(f, "No user input."),
            //ShellErr::DirectoryError(path) => write!(f, "No such directory: {}", path.display()),
        }
    }
}

impl From<io::Error> for ShellErr {
    fn from(err: io::Error) -> Self {
        ShellErr::Io(err)
    }
}

impl std::error::Error for ShellErr {}
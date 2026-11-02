use std::fmt::Display;
use std::io;
use std::path::PathBuf;

// Types of possible errors to handle
#[derive(Debug)]
pub enum ShellErr {
    Io(io::Error),
    DirectoryError(PathBuf),
    EmptyPath,
    UnsupportedCommand(String)
}

// Implement Display for enum members (error types)
// Handling becomes universal for all error types.
// Whenever the implemented error needs printing this will automatically
// Format output to a "canonical" form which can be printed
impl Display for ShellErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellErr::Io(err) => write!(f, "IO error: {}", err),
            ShellErr::DirectoryError(path) => write!(f, "No such directory: {}", path.display()),
            ShellErr::EmptyPath => write!(f, "No file path specified."),
            ShellErr::UnsupportedCommand(cmd) => write!(f, "Unrecognised command: {}", cmd)
        }
    }
}

// Wrap std::io::Error in ShellErr
// So io:Error can be handled alongside other error types
impl From<io::Error> for ShellErr {
    fn from(err: io::Error) -> Self {
        ShellErr::Io(err)
    }
}

// Tell Rust that ShellErr is actually an error type
impl std::error::Error for ShellErr {}
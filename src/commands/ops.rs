use std::env;

use crate::error::ShellErr;

// Print Working Directory
pub fn pwd() -> Result<(), ShellErr> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

// Go one directory up (cd ..)
pub fn parent_dir() -> Result<(), ShellErr> {
    let path = env::current_dir()?;

    // If there is a parent change to it
    if let Some(parent) = path.parent() {
        env::set_current_dir(parent)?;
    }
    // If already at root do nothing

    Ok(())
}
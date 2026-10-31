use std::env;
use std::path::PathBuf;

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

// Change Directory
pub fn change_dir(target_dir: &str) -> Result<(), ShellErr> {
    let path = env::current_dir()?;
    let target_path = PathBuf::from(target_dir);
    
    // Append the target dir to current dir
    // join() already checks whether target is absolute or relative
    // If target is absolute replace path with target path
    let new_path = path.join(target_path);

    // Check if new path exists and is a directory (can't cd to a file)
    if new_path.is_dir() {
        env::set_current_dir(new_path)?;
    }
    else {
        return Err(ShellErr::DirectoryError(new_path))
    }

    Ok(())
}
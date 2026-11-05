use std::env;
use std::path::PathBuf;
use std::fs;

use crate::commands::ls_long;
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

// Change Directory (this also handles ..)
// Technically parent_dir() is redundant
pub fn change_dir(target_dir: &str) -> Result<(), ShellErr> {
    let path = env::current_dir()?;
    let target_path = PathBuf::from(target_dir);
    
    // Append the target dir to current dir
    // join() already checks whether target is absolute or relative
    // Ie checks for / (on Linux at least) 
    // If target is absolute replace path with target path
    let new_path = path.join(target_path);

    // Check if new path exists and is a directory (can't cd to a file)
    if new_path.is_dir() {
        env::set_current_dir(new_path)?;
    }
    else {
        return Err(ShellErr::DirectoryError(new_path));
    }

    Ok(())
}

// List contents of current directory (ls)
pub fn list_dir() -> Result<(), ShellErr> {
    // Iterate over entries in current dir
    for entry in fs::read_dir(".")? {
        // Unwrap DirEntry as there may be io::Error
        // when fetching next entry
        let entry = entry?;
        
        // Get OsString for entry
        let file = entry.file_name();

        // Print directory entry
        // Has to be valid UTF-8, otherwise replace characters with �
        println!("{}", file.to_string_lossy());
    }

    Ok(())
}

// List contents of current directory with more details
pub fn list_dir_long() -> Result<(), ShellErr> {
    for entry in fs::read_dir(".")? {
        // Unwrap DirEntry as there may be io::Error when fetching next entry
        // Fetch metadata for directory entry
        let file_metadata = ls_long::MyMetadata::new(&entry?)?;
        
        // Format output according to display metadata and print
        println!("{}", file_metadata.format_entry());
    }

    Ok(())
}

// List a list of supported functions
pub fn help() {
    print!(r#"
    SIMPLE SHELL
    
    Usage:

    ls              -   list contents of current directory
    ls -l           -   list contents of current directory with more details
    cd [path]       -   move to a specified directory (.. for parent directory)
    pwd             -   print working directory
    mkdir [path]    -   create a new directory at specified path
    write [file]    -   read text from user line-by-line and write to a specified file
    read [file]     -   print contents of a specified file
    help            -   display available commands
    "#);
}

// Bonus commands

// Create a directory at a specific path
pub fn make_directory(dir_path: &str) -> Result<(), ShellErr> {
    // Checks if path exists and if the directory exists
    // Returns io::Error if it does
    fs::create_dir(dir_path)?;
    Ok(())
}


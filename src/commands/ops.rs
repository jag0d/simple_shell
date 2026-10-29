use std::env;

use crate::error::ShellErr;

// Print Working Directory
pub fn pwd() -> Result<(), ShellErr> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}
use std::fs::{self, DirEntry};
use std::os::unix::fs::PermissionsExt;
use std::time::SystemTime;
use std::ffi::OsString;

use crate::error::ShellErr;

// Desired data points about directory entry
pub struct MyMetadata {
    is_dir: bool,
    permissions_mode: u32,
    len_bytes: u64,
    modified: SystemTime,
    file_name: OsString,
}

impl MyMetadata {
    pub fn new(dir: &DirEntry) -> Result<Self, ShellErr> {
        let metadata = dir.metadata()?;
        
        Ok(MyMetadata { 
            is_dir: (metadata.is_dir()),
            permissions_mode: (metadata.permissions().mode()),
            len_bytes: (metadata.len()),
            modified: (metadata.modified()?),
            file_name: (dir.file_name()) 
        })
    }

    pub fn format_permissions(&self) -> String {
        let is_d = if self.is_dir {'d'} else {'-'};

        let root_write = if self.permissions_mode & 0o400 != 0 {'w'} else {'-'};
        let root_read = if self.permissions_mode & 0o200 != 0 {'r'} else {'-'};
        let root_execute = if self.permissions_mode & 0o100 != 0 {'x'} else {'-'};

        let group_write = if self.permissions_mode & 0o040 != 0 {'w'} else {'-'};
        let group_read = if self.permissions_mode & 0o020 != 0 {'r'} else {'-'};
        let group_execute = if self.permissions_mode & 0o010 != 0 {'x'} else {'-'};

        let user_write = if self.permissions_mode & 0o004 != 0 {'w'} else {'-'};
        let user_read = if self.permissions_mode & 0o002 != 0 {'r'} else {'-'};
        let user_execute = if self.permissions_mode & 0o001 != 0 {'x'} else {'-'};

        format!("{}{}{}{}{}{}{}{}{}{}",
            is_d,
            root_write, root_read, root_execute,
            group_write, group_read, group_execute,
            user_write, user_read, user_execute)
    }
}

pub fn list_dir_long() -> Result<(), ShellErr> {
    for entry in fs::read_dir(".")? {
        // Unwrap DirEntry as there may be io::Error
        // when fetching next entry
        let file_metadata = MyMetadata::new(&entry?);
        

    }

    Ok(())
}
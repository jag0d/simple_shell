use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;
use std::time::SystemTime;
use std::ffi::OsString;

use chrono::{DateTime, Local};

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

        let user_read = if self.permissions_mode & 0o400 != 0 {'r'} else {'-'};
        let user_write = if self.permissions_mode & 0o200 != 0 {'w'} else {'-'};
        let user_execute = if self.permissions_mode & 0o100 != 0 {'x'} else {'-'};

        let group_read = if self.permissions_mode & 0o040 != 0 {'r'} else {'-'};
        let group_write = if self.permissions_mode & 0o020 != 0 {'w'} else {'-'};
        let group_execute = if self.permissions_mode & 0o010 != 0 {'x'} else {'-'};

        let other_read = if self.permissions_mode & 0o004 != 0 {'r'} else {'-'};
        let other_write = if self.permissions_mode & 0o002 != 0 {'w'} else {'-'};
        let other_execute = if self.permissions_mode & 0o001 != 0 {'x'} else {'-'};

        format!("{}{}{}{}{}{}{}{}{}{}",
            is_d,
            user_read, user_write, user_execute,
            group_read, group_write, group_execute,
            other_read, other_write, other_execute)
    }

    // Convert SystemTime to a human readable formatted string
    
    pub fn format_datetime(&self) -> String {
        // Convert SystemTime timestamp (just time since UNIX_EPOCH, does not
        // implement Display) into a chrono DateTime timestamp which does
        // implement Display and can be formatted to a string
        let chrono_date_time: DateTime<Local> = self.modified.into();
        chrono_date_time.format("%Y %b %e %H:%M:%S").to_string()
    }

    // Format the relevant directiry entry data points into a string
    pub fn format_entry(&self) -> String {
        format!("{} {:>5}B {} {}", 
            self.format_permissions(),
            self.len_bytes, self.format_datetime(),
            self.file_name.to_string_lossy())
    }
}
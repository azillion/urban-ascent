use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn get_most_recently_changed_file(folder_path: &str) -> io::Result<PathBuf> {
    let folder = Path::new(folder_path);

    let mut most_recent_file: Option<PathBuf> = None;
    let mut most_recent_time: Option<SystemTime> = None;

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let modified_time = metadata.modified()?;

        if let Some(current_time) = most_recent_time {
            if modified_time > current_time {
                most_recent_file = Some(entry.path());
                most_recent_time = Some(modified_time);
            }
        } else {
            most_recent_file = Some(entry.path());
            most_recent_time = Some(modified_time);
        }
    }

    most_recent_file
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No files found in the folder."))
}

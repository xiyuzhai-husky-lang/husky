//! this crate is about a representation of file system, using timestamp for versions

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a file in the virtual file system with its content and timestamp
#[derive(Debug, Clone)]
pub struct TpVfsFile {
    content: Vec<u8>,
    timestamp: u64,
}

/// A virtual file system that tracks file versions using timestamps
#[derive(Debug, Default)]
pub struct TpVfs {
    files: HashMap<PathBuf, TpVfsFile>,
}

impl TpVfs {
    /// Creates a new empty virtual file system
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Gets the file's last modified timestamp in milliseconds
    fn get_timestamp<P: AsRef<Path>>(path: P) -> u64 {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or_else(|_| SystemTime::now())
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    /// Sets or updates a file's content
    pub fn set_file<P: AsRef<Path>>(&mut self, path: P, content: Vec<u8>) {
        self.files.insert(
            path.as_ref().to_path_buf(),
            TpVfsFile {
                content,
                timestamp: Self::get_timestamp(path),
            },
        );
    }

    /// Gets a file's content and timestamp, if it exists
    pub fn get_file<P: AsRef<Path>>(&self, path: P) -> Option<(&[u8], u64)> {
        self.files
            .get(path.as_ref())
            .map(|file| (&file.content[..], file.timestamp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_new_vfs_is_empty() {
        let vfs = TpVfs::new();
        assert!(vfs.files.is_empty());
    }

    #[test]
    fn test_set_and_get_file() {
        let mut vfs = TpVfs::new();
        let content = b"Hello, World!".to_vec();

        vfs.set_file("test.txt", content.clone());

        if let Some((stored_content, _timestamp)) = vfs.get_file("test.txt") {
            assert_eq!(stored_content, content);
        } else {
            panic!("File not found");
        }
    }

    #[test]
    fn test_timestamp_from_real_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");

        fs::write(&file_path, "test content").unwrap();
        let timestamp1 = TpVfs::get_timestamp(&file_path);

        // Small delay to ensure different timestamps
        std::thread::sleep(std::time::Duration::from_millis(10));

        fs::write(&file_path, "updated content").unwrap();
        let timestamp2 = TpVfs::get_timestamp(&file_path);

        assert!(
            timestamp2 > timestamp1,
            "Timestamp should increase after file update"
        );
    }

    #[test]
    fn test_nonexistent_file() {
        let vfs = TpVfs::new();
        assert!(vfs.get_file("nonexistent.txt").is_none());
    }
}

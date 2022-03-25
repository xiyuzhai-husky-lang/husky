use std::path::Path;

// first read and compare, and then write if different
pub fn diff_write(path: &Path, content: &str) {
    let different = match std::fs::read_to_string(path) {
        Ok(content_on_disk) => {
            assert!(content_on_disk.len() > 0);
            content != content_on_disk
        }
        Err(_) => true,
    };
    if different {
        std::fs::write(path, content).unwrap()
    }
}

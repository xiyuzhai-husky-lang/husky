use husky_print_utils::p;
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
        match std::fs::write(path, content) {
            Ok(_) => (),
            Err(e) => {
                p!(path, e);
                todo!()
            }
        }
    }
}

// first read and compare, and then write if different
pub fn diff_copy(src: &Path, dst: &Path) {
    let content = std::fs::read_to_string(src).unwrap();
    diff_write(dst, &content)
}

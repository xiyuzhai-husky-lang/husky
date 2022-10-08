use std::path::{Path, PathBuf};

use interner::{DefaultInternedPtr, Interner};

pub type FilePtr = DefaultInternedPtr<Path, PathBuf>;
pub type FileInterner = Interner<FilePtr>;

pub trait AllocateUniqueFile {
    fn file_interner(&self) -> &FileInterner;

    fn intern_file(&self, path: PathBuf) -> FilePtr {
        self.file_interner()
            .intern(match std::fs::canonicalize(path.clone()) {
                Ok(path) => path,
                Err(_) => path,
            })
    }
}

pub fn new_file_interner() -> FileInterner {
    FileInterner::new_empty()
}

#[test]
fn test_intern_file() {
    use husky_check_utils::*;
    let interner = new_file_interner();
    let path = &*interner.intern("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}

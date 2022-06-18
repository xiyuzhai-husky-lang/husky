use std::path::{Path, PathBuf};

use unique_allocator::UniqueAllocator;
use word::CustomIdentifier;

pub type FileInterner = UniqueAllocator<Path, PathBuf>;
pub type FilePtr = unique_allocator::InternedPtr<Path>;

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

pub fn new_file_unique_allocator() -> FileInterner {
    FileInterner::empty()
}

#[test]
fn test_intern_file() {
    use check_utils::*;
    let unique_allocator = new_file_unique_allocator();
    let path = &*unique_allocator.intern("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}

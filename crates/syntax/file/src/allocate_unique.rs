use std::path::{Path, PathBuf};

use unique_allocator::UniqueAllocator;
use word::CustomIdentifier;

pub type FileInterner = UniqueAllocator<Path, PathBuf>;
pub type FilePtr = unique_allocator::BasicUniqueAllocatorPtr<Path>;

pub trait AllocateUniqueFile {
    fn file_unique_allocator(&self) -> &FileInterner;

    fn intern_file(&self, path: PathBuf) -> FilePtr {
        self.file_unique_allocator()
            .alloc(std::fs::canonicalize(path).unwrap())
    }

    fn submodule_file(&self, module_file: FilePtr, ident: CustomIdentifier) {
        todo!()
    }
}

pub fn new_file_unique_allocator() -> FileInterner {
    FileInterner::empty()
}

#[test]
fn test_intern_file() {
    use check_utils::*;
    let unique_allocator = new_file_unique_allocator();
    let path = &*unique_allocator.alloc("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}

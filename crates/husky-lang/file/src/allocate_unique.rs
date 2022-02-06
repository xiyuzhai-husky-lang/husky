use common::*;
use unique_allocator::UniqueAllocator;

pub type UniqueFileAllocator = UniqueAllocator<Path, PathBuf>;
pub type FilePtr = unique_allocator::BasicUniqueAllocatorPtr<Path>;

pub trait AllocateUniqueFile {
    fn file_unique_allocator(&self) -> &UniqueFileAllocator;

    fn alloc_file(&self, path: PathBuf) -> FilePtr {
        self.file_unique_allocator().alloc(path)
    }
}

pub fn new_file_unique_allocator() -> UniqueFileAllocator {
    UniqueFileAllocator::empty()
}

#[test]
fn test_intern_file() {
    let unique_allocator = new_file_unique_allocator();
    let path = &*unique_allocator.alloc("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}

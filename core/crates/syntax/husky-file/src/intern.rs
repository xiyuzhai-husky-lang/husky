use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use interner::Interner;
use singleton::singleton;
use word::CustomIdentifier;

pub type FileInterner = Interner<Path, PathBuf>;
pub type FilePtr = interner::InternedPtr<Path>;

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

pub fn new_file_interner() -> Arc<FileInternerSingletonKeeper> {
    Arc::new(FileInterner::empty().into())
}

singleton! { FileInterner }

#[test]
fn test_intern_file() {
    use check_utils::*;
    let interner = new_file_interner();
    let path = &*interner.intern("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}

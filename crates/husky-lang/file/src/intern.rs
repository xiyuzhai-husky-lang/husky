use common::*;
use interner::Interner;

pub type FileInterner = Interner<Path, PathBuf>;
pub type FileId = interner::BasicInternId<Path>;

pub trait InternFile {
    fn file_interner(&self) -> &FileInterner;

    fn intern_file(&self, path: PathBuf) -> FileId {
        self.file_interner().intern(path)
    }
}

pub fn new_file_interner() -> FileInterner {
    FileInterner::empty()
}

#[test]
fn test_intern_file() {
    let interner = new_file_interner();
    let path = &*interner.intern("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}

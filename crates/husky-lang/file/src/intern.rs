use common::*;
use interner::Interner;

pub type FileInterner = Interner<Path, PathBuf>;
pub type FileId = interner::BasicInternId<Path>;

pub trait InternFile {
    fn file_interner(&self) -> &FileInterner;

    fn file_id(&self, path: PathBuf) -> FileId {
        self.file_interner().intern(path)
    }
}

pub fn new_file_interner() -> FileInterner {
    FileInterner::empty()
}

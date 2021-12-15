use common::*;
use interner::Interner;

pub type FileInterner = Interner<PathBuf>;
pub type FileId = interner::BasicId<PathBuf>;

pub trait InternFile {
    fn provide_file_interner(&self) -> &FileInterner;

    fn file_id(&self, path: PathBuf) -> FileId {
        self.provide_file_interner().id(path)
    }

    fn file_id_iter(&self) -> interner::IdIter<FileId> {
        self.provide_file_interner().id_iter()
    }
}

pub fn new_file_interner() -> FileInterner {
    FileInterner::new(vec![])
}

pub fn use_filepath<F, Q>(this: &(impl InternFile + ?Sized), id: FileId, f: F) -> Q
where
    F: Fn(&Path) -> Q,
{
    this.provide_file_interner().use_thing(id, f)
}

pub fn snapshot_use_filepath<Database, F, Q>(
    this: &salsa::Snapshot<Database>,
    id: FileId,
    f: F,
) -> Q
where
    F: Fn(&Path) -> Q,
    Database: salsa::ParallelDatabase + InternFile,
{
    use std::ops::Deref;

    this.deref().provide_file_interner().use_thing(id, f)
}

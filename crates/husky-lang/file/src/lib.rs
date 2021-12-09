//! virtual file system  

use std::sync::Arc;

pub use line_map::LineMap;

mod error;
mod line_map;

use common::*;

use interner::Interner;

use stdx::sync::ARwLock;

#[derive(Clone, Copy, Debug)]
pub struct FilePosition {
    pub source_id: FileId,
    pub offset: TextSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FileRange {
    pub source: FileId,
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Main,
    Lib,
    Module,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileContent {
    OnDisk(Arc<String>),
    Live(Arc<String>),
    Deleted,
    NonExistent,
}

pub type FileInterner = Interner<PathBuf>;
pub type FileId = interner::BasicId<PathBuf>;

pub trait InternFile {
    fn provide_source_interner(&self) -> &FileInterner;

    fn source_id(&self, path: PathBuf) -> FileId {
        self.provide_source_interner().id(path)
    }

    fn source_path(&self, id: FileId) -> PathBuf {
        self.provide_source_interner().thing(id)
    }
}

pub trait LiveFiles: InternFile {
    fn get_live_docs(&self) -> &ARwLock<HashMap<FileId, Arc<String>>>;
    fn did_change_source(&mut self, id: FileId);

    fn set_live_doc_content(&mut self, path: PathBuf, content: String) {
        let id = self.source_id(path);
        self.get_live_docs()
            .write(|live_docs| live_docs.insert(id, Arc::new(content)));
        self.did_change_source(id);
    }
}

#[salsa::query_group(FileQueryGroupStorage)]
pub trait FileQueryGroup: std::fmt::Debug + LiveFiles {
    fn file_content(&self, id: FileId) -> FileContent;

    fn main_source_id(&self, module_source_id: FileId) -> Option<FileId>;

    fn line_map(&self, id: FileId) -> LineMap;
}

fn file_content(this: &dyn FileQueryGroup, id: FileId) -> FileContent {
    this.get_live_docs()
        .read(|live_docs| match live_docs.get(&id) {
            Some(text) => FileContent::Live(text.clone()),
            None => {
                let pth = this.source_path(id);
                if pth.is_file() {
                    FileContent::OnDisk(Arc::new(
                        std::fs::read_to_string(this.source_path(id)).expect("io failure"),
                    ))
                } else {
                    FileContent::NonExistent
                }
            }
        })
}

fn main_source_id(this: &dyn FileQueryGroup, module_source_id: FileId) -> Option<FileId> {
    let source_path = this.source_path(module_source_id);
    for ancestor in source_path.ancestors() {
        let id = this.source_id(ancestor.with_file_name("main.hsk"));
        match this.file_content(id) {
            FileContent::OnDisk(_) | FileContent::Live(_) => return Some(id),
            FileContent::Deleted | FileContent::NonExistent => (),
        }
    }
    None
}

fn line_map(this: &dyn FileQueryGroup, id: FileId) -> LineMap {
    todo!()
}

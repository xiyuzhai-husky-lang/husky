//! virtual file system  

use std::sync::Arc;

mod error;

use common::*;
use interner::Interner;
use itertools::Itertools;
use stdx::sync::ARwLock;

#[derive(Clone, Copy, Debug)]
pub struct FilePosition {
    pub file_id: FileId,
    // pub offset: TextSize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FileRange {
    pub source: FileId,
    // pub range: TextRange,
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
    fn provide_file_interner(&self) -> &FileInterner;

    fn file_id(&self, path: PathBuf) -> FileId {
        self.provide_file_interner().id(path)
    }

    fn file_path(&self, id: FileId) -> PathBuf {
        self.provide_file_interner().thing(id)
    }

    fn file_id_iter(&self) -> interner::IdIter<FileId> {
        self.provide_file_interner().id_iter()
    }
}

pub trait LiveFiles: InternFile {
    fn get_live_docs(&self) -> &ARwLock<HashMap<FileId, Arc<String>>>;
    fn did_change_source(&mut self, id: FileId);

    fn set_live_doc_content(&mut self, path: PathBuf, content: String) {
        let id = self.file_id(path);
        self.get_live_docs()
            .write(|live_docs| live_docs.insert(id, Arc::new(content)));
        self.did_change_source(id);
    }
}

#[salsa::query_group(FileQueryStorage)]
pub trait BasicFileQuery: salsa::Database + std::fmt::Debug + LiveFiles {
    fn file_content(&self, id: FileId) -> FileContent;

    fn main_file_id(&self, module_file_id: FileId) -> Option<FileId>;
}

fn file_content(this: &dyn BasicFileQuery, id: FileId) -> FileContent {
    this.salsa_runtime()
        .report_synthetic_read(salsa::Durability::LOW);
    this.get_live_docs()
        .read(|live_docs| match live_docs.get(&id) {
            Some(text) => FileContent::Live(text.clone()),
            None => {
                let pth = this.file_path(id);
                if pth.is_file() {
                    FileContent::OnDisk(Arc::new(
                        std::fs::read_to_string(this.file_path(id)).expect("io failure"),
                    ))
                } else {
                    FileContent::NonExistent
                }
            }
        })
}

fn main_file_id(this: &dyn BasicFileQuery, module_file_id: FileId) -> Option<FileId> {
    let file_path = this.file_path(module_file_id);
    for ancestor in file_path.ancestors() {
        let id = this.file_id(ancestor.with_file_name("main.hsk"));
        match this.file_content(id) {
            FileContent::OnDisk(_) | FileContent::Live(_) => return Some(id),
            FileContent::Deleted | FileContent::NonExistent => (),
        }
    }
    None
}

pub trait FileQuery: BasicFileQuery {
    fn exists_file(&self, id: FileId) -> bool {
        match self.file_content(id) {
            FileContent::OnDisk(_) => true,
            FileContent::Live(_) => true,
            FileContent::Deleted => false,
            FileContent::NonExistent => false,
        }
    }

    fn all_main_files(&self) -> Vec<FileId> {
        self.file_id_iter()
            .filter_map(|id| self.main_file_id(id))
            .unique()
            .collect()
    }

    fn text(&self, id: FileId) -> Option<Arc<String>> {
        match self.file_content(id) {
            FileContent::OnDisk(text) => Some(text),
            FileContent::Live(text) => Some(text),
            FileContent::Deleted => None,
            FileContent::NonExistent => None,
        }
    }
}

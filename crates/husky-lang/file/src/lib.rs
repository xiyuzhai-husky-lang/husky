mod error;
mod intern;
mod line_map;
mod query;
mod utils;

pub use error::{FileError, FileResult, FileResultArc};
pub use intern::{new_file_interner, FileId, FileInterner, InternFile};
pub use query::{FileContentQuery, FileQuery, FileQueryStorage, FileSalsaQuery, LiveFiles};

use std::sync::Arc;

use common::*;

#[derive(Clone, Copy, Debug)]
pub struct FilePosition {
    pub file_id: FileId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileRange {
    pub source: FileId,
    pub range: text::TextRange,
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

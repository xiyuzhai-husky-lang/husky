mod error;
mod intern;
mod line_map;
mod query;
mod utils;

pub use error::{FileError, FileResultArc};
pub use intern::{
    new_file_interner, snapshot_use_filepath, use_filepath, FileId, FileInterner, InternFile,
};
pub use query::{FileContentQuery, FileQuery, FileQueryStorage, FileSalsaQuery, LiveFiles};

use std::sync::Arc;

use stdx::sync::ARwLock;

use common::*;

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

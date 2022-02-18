mod allocate_unique;
mod error;
mod line_map;
mod query;
mod utils;

pub use allocate_unique::{
    new_file_unique_allocator, AllocateUniqueFile, FilePtr, UniqueFileAllocator,
};
pub use error::{FileError, FileResult, FileResultArc};
pub use query::{FileContentQuery, FileQueryGroup, FileQueryStorage, FileSalsaQuery, LiveFiles};

use std::sync::Arc;

use common::*;

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

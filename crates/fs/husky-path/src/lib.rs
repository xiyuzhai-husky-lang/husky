mod error;
mod intern;
mod line_map;
#[cfg(feature = "lsp_support")]
mod lsp_support;
mod query;
mod utils;

pub use error::*;
pub use intern::InternHuskyPath;
pub use interner::path::{InternPath, PathInterner, PathItd};
pub use query::{
    FileContentQuery, FileQueryGroup, FileQueryStorage, FileSalsaQuery, LiveFileSupport,
    VfsQueryGroupBase,
};
pub type URange = std::ops::Range<usize>;

use std::sync::Arc;

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

impl FileContent {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            FileContent::OnDisk(s) => Some(&s),
            FileContent::Live(s) => Some(&s),
            FileContent::Deleted | FileContent::NonExistent => None,
        }
    }
}

pub struct ModulePathItd(PathItd);

impl ModulePathItd {
    fn new(path: PathItd) -> Option<Self> {
        todo!();
        Some(ModulePathItd(path))
    }
}

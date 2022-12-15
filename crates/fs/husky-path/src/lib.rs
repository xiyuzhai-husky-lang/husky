mod error;
mod line_map;
#[cfg(feature = "lsp_support")]
mod lsp_support;
mod utils;

pub use error::*;
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

// pub struct ModuleSourcePath(AbsolutePath);

// impl ModuleSourcePath {
//     fn new(path: AbsolutePath) -> Option<Self> {
//         todo!();
//         Some(ModuleSourcePath(path))
//     }
// }
